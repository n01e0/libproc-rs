extern crate libc;

use crate::libproc::file_info::{PIDFDInfo, PIDFDInfoFlavor};

use self::libc::{c_char, c_int, c_short, c_uchar, c_ushort, gid_t, IF_NAMESIZE, in6_addr, in_addr,
                 off_t, SOCK_MAXADDRLEN, sockaddr_un, uid_t};

/// Socket File Descriptor Info
#[repr(C)]
#[derive(Default)]
pub struct SocketFDInfo {
    /// Proc File Info
    pub pfi: ProcFileInfo,
    /// Socket Info
    pub psi: SocketInfo,
}

/// Proc File Info
#[repr(C)]
#[derive(Default)]
pub struct ProcFileInfo {
    /// Open flags
    pub fi_openflags: u32,
    /// Status
    pub fi_status   : u32,
    /// Offset
    pub fi_offset   : off_t,
    /// Type
    pub fi_type     : i32,
    /// Reserved for future use
    pub rfu_1       : i32,
}

impl PIDFDInfo for SocketFDInfo {
    fn flavor() -> PIDFDInfoFlavor { PIDFDInfoFlavor::SocketInfo }
}

/// Socket Info Kind
#[derive(Copy, Clone, Debug)]
pub enum SocketInfoKind {
    /// Generic
    Generic = 0,
    /// IPv4 and IPv6 Sockets
    In = 1,
    /// TCP Sockets
    Tcp = 2,
    /// Unix Domain Sockets
    Un = 3,
    /// PF_NDRV Sockets
    Ndrv = 4,
    /// Kernel Event Sockets
    KernEvent = 5,
    /// Kernel Control Sockets
    KernCtl = 6,
    /// Unknown
    Unknown,
}

impl From<c_int> for SocketInfoKind {
    fn from(value: c_int) -> SocketInfoKind {
        match value {
            0 => SocketInfoKind::Generic,
            1 => SocketInfoKind::In,
            2 => SocketInfoKind::Tcp,
            3 => SocketInfoKind::Un,
            4 => SocketInfoKind::Ndrv,
            5 => SocketInfoKind::KernEvent,
            6 => SocketInfoKind::KernCtl,
            _ => SocketInfoKind::Unknown,
        }
    }
}

/// Socket Info
#[repr(C)]
#[derive(Default)]
pub struct SocketInfo {
    /// Stat
    pub soi_stat: VInfoStat,
    /// SO
    pub soi_so: u64,
    /// PCB
    pub soi_pcb: u64,
    /// Type
    pub soi_type: c_int,
    /// Protocol
    pub soi_protocol: c_int,
    /// Family
    pub soi_family: c_int,
    /// Options
    pub soi_options: c_short,
    /// Linger
    pub soi_linger: c_short,
    /// State
    pub soi_state: c_short,
    /// Queue Length
    pub soi_qlen: c_short,
    /// Incremental Queue Length
    pub soi_incqlen: c_short,
    /// Queue Limit
    pub soi_qlimit: c_short,
    /// Time O
    pub soi_timeo: c_short,
    /// Error
    pub soi_error: c_ushort,
    /// OOB Mark
    pub soi_oobmark: u32,
    /// Receive
    pub soi_rcv: SockBufInfo,
    /// Send
    pub soi_snd: SockBufInfo,
    /// Kind
    pub soi_kind: c_int,
    /// Reserved for future use
    pub rfu_1: u32,
    /// Proto
    pub soi_proto: SocketInfoProto,
}

/// Struct for V Info Stat
#[repr(C)]
#[derive(Default)]
pub struct VInfoStat {
    /// Dev
    pub vst_dev: u32,
    /// Mode
    pub vst_mode: u16,
    /// Nlink
    pub vst_nlink: u16,
    /// Ino
    pub vst_ino: u64,
    /// UID
    pub vst_uid: uid_t,
    /// GID
    pub vst_gid: gid_t,
    /// ATime
    pub vst_atime: i64,
    /// ATime nano seconds
    pub vst_atimensec: i64,
    /// MTime
    pub vst_mtime: i64,
    /// MTime nano seconds
    pub vst_mtimensec: i64,
    /// CTime
    pub vst_ctime: i64,
    /// CTime nano seconds
    pub vst_ctimensec: i64,
    /// Birthtime
    pub vst_birthtime: i64,
    /// Birthtime nano seconds
    pub vst_birthtimensec: i64,
    /// Size
    pub vst_size: off_t,
    /// Blocks
    pub vst_blocks: i64,
    /// Block Size
    pub vst_blksize: i32,
    /// Flags
    pub vst_flags: u32,
    /// Gen
    pub vst_gen: u32,
    /// RDev
    pub vst_rdev: u32,
    /// Q spare
    pub vst_qspare: [i64; 2],
}

/// Socket Buffer Info
#[repr(C)]
#[derive(Default)]
pub struct SockBufInfo {
    /// CC
    pub sbi_cc: u32,
    /// HiWat
    pub sbi_hiwat: u32,
    /// MB Count
    pub sbi_mbcnt: u32,
    /// MB Max
    pub sbi_mbmax: u32,
    /// Lowat
    pub sbi_lowat: u32,
    /// Flags
    pub sbi_flags: c_short,
    /// Timeo
    pub sbi_timeo: c_short,
}

/// Socket Info Proto
#[repr(C)]
pub union SocketInfoProto {
    /// In socket info
    pub pri_in: InSockInfo,
    /// TCP Socket Info
    pub pri_tcp: TcpSockInfo,
    /// Un socket info
    pub pri_un: UnSockInfo,
    /// N Drive Info
    pub pri_ndrv: NdrvInfo,
    /// KernEventInfo
    pub pri_kern_event: KernEventInfo,
    /// Kernel Control Info
    pub pri_kern_ctl: KernCtlInfo,
}

impl Default for SocketInfoProto {
    fn default() -> SocketInfoProto {
        SocketInfoProto {
            pri_in: Default::default(),
        }
    }
}

/// In4In6Addr struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct In4In6Addr {
    /// Padding
    pub i46a_pad32: [u32; 3],
    /// Address
    pub i46a_addr4: in_addr,
}

impl Default for In4In6Addr {
    fn default() -> In4In6Addr {
        In4In6Addr {
            i46a_pad32: [0; 3],
            i46a_addr4: in_addr { s_addr: 0 },
        }
    }
}

/// InSocket info struct
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct InSockInfo {
    /// F Port
    pub insi_fport: c_int,
    /// L Port
    pub insi_lport: c_int,
    /// General Count
    pub insi_gencnt: u64,
    /// Flags
    pub insi_flags: u32,
    /// Flow
    pub insi_flow: u32,
    /// VFlag
    pub insi_vflag: u8,
    /// IP TTL
    pub insi_ip_ttl: u8,
    /// Reserved for future use
    pub rfu_1: u32,
    /// F Address
    pub insi_faddr: InSIAddr,
    /// L Address
    pub insi_laddr: InSIAddr,
    /// V4 info
    pub insi_v4: InSIV4,
    /// V6 info
    pub insi_v6: InSIV6,
}

/// InSIV4 struct
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct InSIV4 {
    /// InV4 top
    pub in4_top: c_uchar,
}

/// InSIV6 struct
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct InSIV6 {
    /// Hlim
    pub in6_hlim: u8,
    /// Checksum
    pub in6_cksum: c_int,
    /// Interface Index
    pub in6_ifindex: c_ushort,
    /// Hops
    pub in6_hops: c_short,
}

/// InSIAddress union for v4 and v6 addresses
#[repr(C)]
#[derive(Copy, Clone)]
pub union InSIAddr {
    /// v4 address
    pub ina_46: In4In6Addr,
    /// v6 address
    pub ina_6: in6_addr,
}

impl Default for InSIAddr {
    fn default() -> InSIAddr {
        InSIAddr {
            ina_46: Default::default(),
        }
    }
}

/// TCP SI State struct
#[derive(Copy, Clone, Debug)]
pub enum TcpSIState {
    /// Closed
    Closed = 0,
    /// Listening for connection
    Listen = 1,
    /// Active, have sent syn
    SynSent = 2,
    /// Have send and received syn
    SynReceived = 3,
    /// Established
    Established = 4,
    /// Rcvd fin, waiting for close
    CloseWait = 5,
    /// Have closed, sent fin
    FinWait1 = 6,
    /// Closed xchd FIN; await FIN ACK
    Closing = 7,
    /// Had fin and close; await FIN ACK
    LastAck = 8,
    /// Have closed, fin is acked
    FinWait2 = 9,
    /// In 2*msl quiet wait after close
    TimeWait = 10,
    /// Pseudo state: reserved
    Reserved = 11,
    /// Unknown
    Unknown,
}

impl From<c_int> for TcpSIState {
    fn from(value: c_int) -> TcpSIState {
        match value {
            0 => TcpSIState::Closed,
            1 => TcpSIState::Listen,
            2 => TcpSIState::SynSent,
            3 => TcpSIState::SynReceived,
            4 => TcpSIState::Established,
            5 => TcpSIState::CloseWait,
            6 => TcpSIState::FinWait1,
            7 => TcpSIState::Closing,
            8 => TcpSIState::LastAck,
            9 => TcpSIState::FinWait2,
            10 => TcpSIState::TimeWait,
            11 => TcpSIState::Reserved,
            _ => TcpSIState::Unknown,
        }
    }
}

const TSI_T_NTIMERS: usize = 4;

/// TCP Socket Info struct
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct TcpSockInfo {
    /// In Socket Info
    pub tcpsi_ini: InSockInfo,
    /// State
    pub tcpsi_state: c_int,
    /// Timer
    pub tcpsi_timer: [c_int; TSI_T_NTIMERS],
    /// MSS
    pub tcpsi_mss: c_int,
    /// Flags
    pub tcpsi_flags: u32,
    /// Reserved for future use
    pub rfu_1: u32,
    /// TP
    pub tcpsi_tp: u64,
}

/// Un Socket Info struct
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct UnSockInfo {
    /// Conn SO
    pub unsi_conn_so: u64,
    /// Conn PCB
    pub unsi_conn_pcb: u64,
    /// Address
    pub unsi_addr: UnSIAddr,
    /// CAddress
    pub unsi_caddr: UnSIAddr,
}

/// UnSI Address struct
#[repr(C)]
#[derive(Copy, Clone)]
pub union UnSIAddr {
    /// Socket address Un
    pub ua_sun: sockaddr_un,
    /// Dummy
    pub ua_dummy: [c_char; SOCK_MAXADDRLEN as usize],
}

impl Default for UnSIAddr {
    fn default() -> UnSIAddr {
        UnSIAddr {
            ua_dummy: [0; SOCK_MAXADDRLEN as usize],
        }
    }
}

/// NDrvInfo struct
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct NdrvInfo {
    /// Interface Family
    pub ndrvsi_if_family: u32,
    /// Interface Unit
    pub ndrvsi_if_unit: u32,
    /// Interface name
    pub ndrvsi_if_name: [c_char; IF_NAMESIZE],
}

/// Kernel Event Info struct
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct KernEventInfo {
    /// Vendor code filter
    pub kesi_vendor_code_filter: u32,
    /// Class filter
    pub kesi_class_filter: u32,
    /// Subclass filter
    pub kesi_subclass_filter: u32,
}

const MAX_KCTL_NAME: usize = 96;

/// Kernel Control Info struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct KernCtlInfo {
    /// ID
    pub kcsi_id: u32,
    /// Reg Unit
    pub kcsi_reg_unit: u32,
    /// Flags
    pub kcsi_flags: u32,
    /// Receive Buffer Size
    pub kcsi_recvbufsize: u32,
    /// Send Buffer Size
    pub kcsi_sendbufsize: u32,
    /// Unit
    pub kcsi_unit: u32,
    /// Name
    pub kcsi_name: [c_char; MAX_KCTL_NAME],
}

impl Default for KernCtlInfo {
    fn default() -> KernCtlInfo {
        KernCtlInfo {
            kcsi_id: 0,
            kcsi_reg_unit: 0,
            kcsi_flags: 0,
            kcsi_recvbufsize: 0,
            kcsi_sendbufsize: 0,
            kcsi_unit: 0,
            kcsi_name: [0; MAX_KCTL_NAME],
        }
    }
}