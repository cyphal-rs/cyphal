#![no_std]
#![no_main]

extern crate alloc;

use cyphal_can::FD_PAYLOAD_SIZE;
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::can::{Can, Instance};
use embassy_stm32::peripherals::*;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, can, Config};
use embassy_time::Timer;
use embedded_alloc::Heap;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    FDCAN1_IT0 => can::IT0InterruptHandler<FDCAN1>;
    FDCAN1_IT1 => can::IT1InterruptHandler<FDCAN1>;
});

#[global_allocator]
static HEAP: Heap = Heap::empty();

struct CyphalCan<'d, I>
where
    I: Instance,
{
    can: Can<'d, I>,
}

impl<'d, I> cyphal_can::Can<FD_PAYLOAD_SIZE> for CyphalCan<'d, I>
where
    I: Instance,
{
    type Frame = CyphalFrame;

    async fn transmit(&mut self, frame: &Self::Frame) -> cyphal_can::CanResult<()> {
        let _result = self.can.write(frame.inner_frame()).await;
        Ok(())
    }

    async fn receive(&mut self) -> cyphal_can::CanResult<Self::Frame> {
        match self.can.read().await {
            Ok(env) => {
                let id: u32 = match env.frame.id() {
                    embedded_can::Id::Standard(_) => return Err(cyphal_can::CanError::Other),
                    embedded_can::Id::Extended(ext) => ext.as_raw(),
                };
                let can_id = cyphal_can::CanId::new(id)?;
                Self::Frame::new(can_id, env.frame.data())
            }
            Err(_) => Err(cyphal_can::CanError::Other),
        }
    }
}

pub struct CyphalFrame {
    can_id: cyphal_can::CanId,
    frame: embassy_stm32::can::Frame,
}

impl CyphalFrame {
    pub(crate) fn inner_frame(&self) -> &embassy_stm32::can::Frame {
        &self.frame
    }
}

use cyphal_can::Frame as _;

impl cyphal_can::Frame<FD_PAYLOAD_SIZE> for CyphalFrame {
    fn new(id: impl Into<cyphal_can::CanId>, data: &[u8]) -> cyphal_can::CanResult<Self> {
        let can_id: cyphal_can::CanId = id.into();
        match data.len() {
            n if n <= FD_PAYLOAD_SIZE => {
                let mut bytes: [u8; FD_PAYLOAD_SIZE] = [0; FD_PAYLOAD_SIZE];
                bytes[..n].copy_from_slice(data);
                let frame = embassy_stm32::can::Frame::new_extended(can_id.as_raw(), data).unwrap();
                Ok(Self { can_id, frame })
            }
            _ => Err(cyphal_can::CanError::Other),
        }
    }

    fn id(&self) -> cyphal_can::CanId {
        self.can_id
    }

    fn dlc(&self) -> usize {
        self.frame.data().len()
    }

    fn data(&self) -> &[u8] {
        self.frame.data()
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }

    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(24_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV6,
            mul: PllMul::MUL85,
            divp: None,
            divq: Some(PllQDiv::DIV8), // 42.5 Mhz for fdcan.
            divr: Some(PllRDiv::DIV2), // Main system clock at 170 MHz
        });
        config.rcc.mux.fdcansel = mux::Fdcansel::PLL1_Q;
        config.rcc.sys = Sysclk::PLL1_R;
    }
    let peripherals = embassy_stm32::init(config);

    let mut can =
        can::CanConfigurator::new(peripherals.FDCAN1, peripherals.PA11, peripherals.PA12, Irqs);

    can.properties().set_extended_filter(
        can::filter::ExtendedFilterSlot::_0,
        can::filter::ExtendedFilter::accept_all_into_fifo1(),
    );

    // 250k bps
    can.set_bitrate(250_000);

    let use_fd = false;

    // 1M bps
    if use_fd {
        can.set_fd_data_bitrate(1_000_000, false);
    }

    info!("Configured");

    let mut can = can.start(match use_fd {
        true => can::OperatingMode::InternalLoopbackMode,
        false => can::OperatingMode::NormalOperationMode,
    });

    let mut i = 0;
    let mut last_read_ts = embassy_time::Instant::now();

    loop {
        let frame = can::frame::Frame::new_extended(0x123456F, &[i; 8]).unwrap();
        info!("Writing frame");

        _ = can.write(&frame).await;

        match can.read().await {
            Ok(envelope) => {
                let (ts, rx_frame) = (envelope.ts, envelope.frame);
                let delta = (ts - last_read_ts).as_millis();
                last_read_ts = ts;
                info!(
                    "Rx: {} {:02x} --- {}ms",
                    rx_frame.header().len(),
                    rx_frame.data()[0..rx_frame.header().len() as usize],
                    delta,
                )
            }
            Err(_err) => error!("Error in frame"),
        }

        Timer::after_millis(250).await;

        i += 1;
        if i > 2 {
            break;
        }
    }

    // Use the FD API's even if we don't get FD packets.

    loop {
        if use_fd {
            let frame = can::frame::FdFrame::new_extended(0x123456F, &[i; 16]).unwrap();
            info!("Writing frame using FD API");
            _ = can.write_fd(&frame).await;
        } else {
            let frame = can::frame::FdFrame::new_extended(0x123456F, &[i; 8]).unwrap();
            info!("Writing frame using FD API");
            _ = can.write_fd(&frame).await;
        }

        match can.read_fd().await {
            Ok(envelope) => {
                let (ts, rx_frame) = (envelope.ts, envelope.frame);
                let delta = (ts - last_read_ts).as_millis();
                last_read_ts = ts;
                info!(
                    "Rx: {} {:02x} --- using FD API {}ms",
                    rx_frame.header().len(),
                    rx_frame.data()[0..rx_frame.header().len() as usize],
                    delta,
                )
            }
            Err(_err) => error!("Error in frame"),
        }

        Timer::after_millis(250).await;

        i += 1;
        if i > 4 {
            break;
        }
    }
    i = 0;
    let (mut tx, mut rx, _props) = can.split();
    // With split
    loop {
        let frame = can::frame::Frame::new_extended(0x123456F, &[i; 8]).unwrap();
        info!("Writing frame");
        _ = tx.write(&frame).await;

        match rx.read().await {
            Ok(envelope) => {
                let (ts, rx_frame) = (envelope.ts, envelope.frame);
                let delta = (ts - last_read_ts).as_millis();
                last_read_ts = ts;
                info!(
                    "Rx: {} {:02x} --- {}ms",
                    rx_frame.header().len(),
                    rx_frame.data()[0..rx_frame.header().len() as usize],
                    delta,
                )
            }
            Err(_err) => error!("Error in frame"),
        }

        Timer::after_millis(250).await;

        i += 1;

        if i > 2 {
            break;
        }
    }

    let can = can::Can::join(tx, rx);

    info!("\n\n\nBuffered\n");
    if use_fd {
        static TX_BUF: StaticCell<can::TxFdBuf<8>> = StaticCell::new();
        static RX_BUF: StaticCell<can::RxFdBuf<10>> = StaticCell::new();
        let mut can = can.buffered_fd(
            TX_BUF.init(can::TxFdBuf::<8>::new()),
            RX_BUF.init(can::RxFdBuf::<10>::new()),
        );
        loop {
            let frame = can::frame::FdFrame::new_extended(0x123456F, &[i; 16]).unwrap();
            info!("Writing frame");

            _ = can.write(frame).await;

            match can.read().await {
                Ok(envelope) => {
                    let (ts, rx_frame) = (envelope.ts, envelope.frame);
                    let delta = (ts - last_read_ts).as_millis();
                    last_read_ts = ts;
                    info!(
                        "Rx: {} {:02x} --- {}ms",
                        rx_frame.header().len(),
                        rx_frame.data()[0..rx_frame.header().len() as usize],
                        delta,
                    )
                }
                Err(_err) => error!("Error in frame"),
            }

            Timer::after_millis(250).await;

            i = i.wrapping_add(1);
        }
    } else {
        static TX_BUF: StaticCell<can::TxBuf<8>> = StaticCell::new();
        static RX_BUF: StaticCell<can::RxBuf<10>> = StaticCell::new();
        let mut can = can.buffered(
            TX_BUF.init(can::TxBuf::<8>::new()),
            RX_BUF.init(can::RxBuf::<10>::new()),
        );
        loop {
            let frame = can::frame::Frame::new_extended(0x123456F, &[i; 8]).unwrap();
            info!("Writing frame");

            // You can use any of these approaches to send. The writer makes it
            // easy to share sending from multiple tasks.
            //_ = can.write(frame).await;
            //can.writer().try_write(frame).unwrap();
            can.writer().write(frame).await;

            match can.read().await {
                Ok(envelope) => {
                    let (ts, rx_frame) = (envelope.ts, envelope.frame);
                    let delta = (ts - last_read_ts).as_millis();
                    last_read_ts = ts;
                    info!(
                        "Rx: {} {:02x} --- {}ms",
                        rx_frame.header().len(),
                        rx_frame.data()[0..rx_frame.header().len() as usize],
                        delta,
                    )
                }
                Err(_err) => error!("Error in frame"),
            }

            Timer::after_millis(250).await;

            i = i.wrapping_add(1);
        }
    }
}
