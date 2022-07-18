#[cfg(target_arch = "x86_64")]
mod x86_64 {
    use crate::arch::x86_64::io::byte::{io_in, io_out};

    #[derive(Clone, Copy)]
    #[repr(u16)]
    pub enum COMPort {
        COM1 = 0x3F8,
        COM2 = 0x2F8,
        COM3 = 0x3E8,
        COM4 = 0x2E8,
        COM5 = 0x5F8,
        COM6 = 0x4F8,
        COM7 = 0x5E8,
        COM8 = 0x4E8,
    }

    #[derive(Clone, Copy)]
    struct ActiveCOMPorts {
        pub baud_rate: u32,
        pub currently_active: u32,
    }

    static mut com_ports: [Option<ActiveCOMPorts>; 8] = [None; 8];

    pub unsafe fn com_port_add(port: COMPort, baud_rate: u32) {
        let com_port = match port {
            COMPort::COM1 => &mut com_ports[0],
            COMPort::COM2 => &mut com_ports[1],
            COMPort::COM3 => &mut com_ports[2],
            COMPort::COM4 => &mut com_ports[3],
            COMPort::COM5 => &mut com_ports[4],
            COMPort::COM6 => &mut com_ports[5],
            COMPort::COM7 => &mut com_ports[6],
            COMPort::COM8 => &mut com_ports[7],
        };
        if let Some(com_port) = com_port {
            if com_port.baud_rate == baud_rate {
                com_port.currently_active += 1;
            } else {
                panic!("Attempted to add a new handle to COM port with a different baud rate.")
            }
        } else {
            let port = port as u16;
            io_out(port + 1, 0x00);
            io_out(port + 3, 0x80);
            io_out(port + 0, 0x03);
            io_out(port + 1, 0x00);
            io_out(port + 3, 0x03);
            io_out(port + 2, 0xC7);
            io_out(port + 4, 0x0B);
            io_out(port + 4, 0x1E);

            io_out(port, 0xEA);
            if io_in(port) != 0xEA {
                panic!("Unabel to properly configure COM port for serial.")
            }

            io_out(port + 4, 0x0F);

            *com_port = Some(ActiveCOMPorts {
                baud_rate,
                currently_active: 1,
            })
        }
    }

    pub unsafe fn com_port_remove(port: COMPort) {
        let com_port = match port {
            COMPort::COM1 => &mut com_ports[0],
            COMPort::COM2 => &mut com_ports[1],
            COMPort::COM3 => &mut com_ports[2],
            COMPort::COM4 => &mut com_ports[3],
            COMPort::COM5 => &mut com_ports[4],
            COMPort::COM6 => &mut com_ports[5],
            COMPort::COM7 => &mut com_ports[6],
            COMPort::COM8 => &mut com_ports[7],
        };
        if let Some(port) = com_port {
            port.currently_active -= 1;
            if port.currently_active == 0 {
                *com_port = None;
            }
        } else {
            panic!(
                "Attempted to remove COM port handle when there was no COM port handle to remove."
            )
        }
    }
}

#[cfg(target_arch = "x86_64")]
struct Data {
    port: x86_64::COMPort,
    baud_rate: u32,
}

pub struct UART(Data);

impl UART {
    #[cfg(target_arch = "x86_64")]
    pub unsafe fn new(port: x86_64::COMPort, baud_rate: u32) -> Option<Self> {
        x86_64::com_port_add(port, baud_rate);
        Some(Self(Data { port, baud_rate }))
    }
}

unsafe impl crate::UnsafeDefault for UART {
    #[cfg(target_arch = "x86_64")]
    unsafe fn default() -> Self {
        Self::new(x86_64::COMPort::COM1, 115200).unwrap()
    }
}

impl Drop for UART {
    #[cfg(target_arch = "x86_64")]
    fn drop(&mut self) {
        unsafe {
            x86_64::com_port_remove(self.0.port);
        }
    }
}

unsafe impl crate::hal::serial::ByteSerial for UART {
    #[cfg(target_arch = "x86_64")]
    unsafe fn putb(&self, byte: u8) {
        use crate::arch::x86_64::io::byte::io_out;
        io_out(self.0.port as u16, byte);
    }

    #[cfg(target_arch = "x86_64")]
    unsafe fn getb(&self) -> u8 {
        use crate::arch::x86_64::io::byte::io_in;
        io_in(self.0.port as u16)
    }
}
