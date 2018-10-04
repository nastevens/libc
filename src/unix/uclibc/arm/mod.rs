//! Definitions for uclibc on 32bit arm systems

use dox::{mem, Option};

pub type c_char = u8;
pub type wchar_t = u32;

s! {
    pub struct ipc_perm {
        pub __key: ::key_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
        pub cuid: ::uid_t,
        pub cgid: ::gid_t,
        pub mode: ::c_ushort,
        __pad1: ::c_ushort,
        pub __seq: ::c_ushort,
        __pad2: ::c_ushort,
        __unused1: ::c_ulong,
        __unused2: ::c_ulong
    }

    pub struct stat64 {
        pub st_dev: ::dev_t,
        __pad1: ::c_uint,
        __st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        __pad2: ::c_uint,
        pub st_size: ::off64_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt64_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_ino: ::ino64_t,
    }

    pub struct statfs64 {
        pub f_type: ::c_int,
        pub f_bsize: ::c_int,
        pub f_blocks: u64,
        pub f_bfree: u64,
        pub f_bavail: u64,
        pub f_files: u64,
        pub f_ffree: u64,
        pub f_fsid: ::fsid_t,
        pub f_namelen: ::c_int,
        pub f_frsize: ::c_int,
        pub f_spare: [::c_int; 5],
    }

    pub struct statvfs64 {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: u64,
        pub f_bfree: u64,
        pub f_bavail: u64,
        pub f_files: u64,
        pub f_ffree: u64,
        pub f_favail: u64,
        pub f_fsid: ::c_ulong,
        __f_unused: ::c_int,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        __f_spare: [::c_int; 6],
    }

    pub struct shmid_ds {
        pub shm_perm: ::ipc_perm,
        pub shm_segsz: ::size_t,
        pub shm_atime: ::time_t,
        __unused1: ::c_ulong,
        pub shm_dtime: ::time_t,
        __unused2: ::c_ulong,
        pub shm_ctime: ::time_t,
        __unused3: ::c_ulong,
        pub shm_cpid: ::pid_t,
        pub shm_lpid: ::pid_t,
        pub shm_nattch: ::shmatt_t,
        __unused4: ::c_ulong,
        __unused5: ::c_ulong
    }

    pub struct msqid_ds {
        pub msg_perm: ::ipc_perm,
        pub msg_stime: ::time_t,
        __glibc_reserved1: ::c_ulong,
        pub msg_rtime: ::time_t,
        __glibc_reserved2: ::c_ulong,
        pub msg_ctime: ::time_t,
        __glibc_reserved3: ::c_ulong,
        __msg_cbytes: ::c_ulong,
        pub msg_qnum: ::msgqnum_t,
        pub msg_qbytes: ::msglen_t,
        pub msg_lspid: ::pid_t,
        pub msg_lrpid: ::pid_t,
        __glibc_reserved4: ::c_ulong,
        __glibc_reserved5: ::c_ulong,
    }

    pub struct termios2 {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_line: ::cc_t,
        pub c_cc: [::cc_t; 19],
        pub c_ispeed: ::speed_t,
        pub c_ospeed: ::speed_t,
    }
}

pub const O_DIRECT: ::c_int = 0x10000;
pub const O_DIRECTORY: ::c_int = 0x4000;
pub const O_NOFOLLOW: ::c_int = 0x8000;
pub const O_LARGEFILE: ::c_int = 0o400000;

pub const MAP_LOCKED: ::c_int = 0x02000;
pub const MAP_NORESERVE: ::c_int = 0x04000;

pub const EDEADLOCK: ::c_int = 35;

pub const SO_PASSCRED: ::c_int = 16;
pub const SO_PEERCRED: ::c_int = 17;
pub const SO_RCVLOWAT: ::c_int = 18;
pub const SO_SNDLOWAT: ::c_int = 19;
pub const SO_RCVTIMEO: ::c_int = 20;
pub const SO_SNDTIMEO: ::c_int = 21;
pub const SO_SNDBUFFORCE: ::c_int = 32;
pub const SO_RCVBUFFORCE: ::c_int = 33;

pub const FIOCLEX: ::c_ulong = 0x5451;
pub const FIONBIO: ::c_ulong = 0x5421;

pub const PTRACE_GETFPXREGS: ::c_uint = 18;
pub const PTRACE_SETFPXREGS: ::c_uint = 19;

pub const MCL_CURRENT: ::c_int = 0x0001;
pub const MCL_FUTURE: ::c_int = 0x0002;

pub const SIGSTKSZ: ::size_t = 8192;
pub const MINSIGSTKSZ: ::size_t = 2048;
pub const CBAUD: ::tcflag_t = 0o0010017;
pub const TAB1: ::c_int = 0x00000800;
pub const TAB2: ::c_int = 0x00001000;
pub const TAB3: ::c_int = 0x00001800;
pub const CR1: ::c_int  = 0x00000200;
pub const CR2: ::c_int  = 0x00000400;
pub const CR3: ::c_int  = 0x00000600;
pub const FF1: ::c_int  = 0x00008000;
pub const BS1: ::c_int  = 0x00002000;
pub const VT1: ::c_int  = 0x00004000;
pub const VWERASE: usize = 14;
pub const VREPRINT: usize = 12;
pub const VSUSP: usize = 10;
pub const VSTART: usize = 8;
pub const VSTOP: usize = 9;
pub const VDISCARD: usize = 13;
pub const VTIME: usize = 5;
pub const IXON: ::tcflag_t = 0x00000400;
pub const IXOFF: ::tcflag_t = 0x00001000;
pub const ONLCR: ::tcflag_t = 0x4;
pub const CSIZE: ::tcflag_t = 0x00000030;
pub const CS6: ::tcflag_t = 0x00000010;
pub const CS7: ::tcflag_t = 0x00000020;
pub const CS8: ::tcflag_t = 0x00000030;
pub const CSTOPB: ::tcflag_t = 0x00000040;
pub const CREAD: ::tcflag_t = 0x00000080;
pub const PARENB: ::tcflag_t = 0x00000100;
pub const PARODD: ::tcflag_t = 0x00000200;
pub const HUPCL: ::tcflag_t = 0x00000400;
pub const CLOCAL: ::tcflag_t = 0x00000800;
pub const ECHOKE: ::tcflag_t = 0x00000800;
pub const ECHOE: ::tcflag_t = 0x00000010;
pub const ECHOK: ::tcflag_t = 0x00000020;
pub const ECHONL: ::tcflag_t = 0x00000040;
pub const ECHOPRT: ::tcflag_t = 0x00000400;
pub const ECHOCTL: ::tcflag_t = 0x00000200;
pub const ISIG: ::tcflag_t = 0x00000001;
pub const ICANON: ::tcflag_t = 0x00000002;
pub const PENDIN: ::tcflag_t = 0x00004000;
pub const NOFLSH: ::tcflag_t = 0x00000080;
pub const CIBAUD: ::tcflag_t = 0o02003600000;
pub const CBAUDEX: ::tcflag_t = 0o010000;
pub const VSWTC: usize = 7;
pub const OLCUC:  ::tcflag_t = 0o000002;
pub const NLDLY:  ::tcflag_t = 0o000400;
pub const CRDLY:  ::tcflag_t = 0o003000;
pub const TABDLY: ::tcflag_t = 0o014000;
pub const BSDLY:  ::tcflag_t = 0o020000;
pub const FFDLY:  ::tcflag_t = 0o100000;
pub const VTDLY:  ::tcflag_t = 0o040000;
pub const XTABS:  ::tcflag_t = 0o014000;

pub const B0: ::speed_t = 0o000000;
pub const B50: ::speed_t = 0o000001;
pub const B75: ::speed_t = 0o000002;
pub const B110: ::speed_t = 0o000003;
pub const B134: ::speed_t = 0o000004;
pub const B150: ::speed_t = 0o000005;
pub const B200: ::speed_t = 0o000006;
pub const B300: ::speed_t = 0o000007;
pub const B600: ::speed_t = 0o000010;
pub const B1200: ::speed_t = 0o000011;
pub const B1800: ::speed_t = 0o000012;
pub const B2400: ::speed_t = 0o000013;
pub const B4800: ::speed_t = 0o000014;
pub const B9600: ::speed_t = 0o000015;
pub const B19200: ::speed_t = 0o000016;
pub const B38400: ::speed_t = 0o000017;
pub const EXTA: ::speed_t = B19200;
pub const EXTB: ::speed_t = B38400;
pub const BOTHER: ::speed_t = 0o010000;
pub const B57600: ::speed_t = 0o010001;
pub const B115200: ::speed_t = 0o010002;
pub const B230400: ::speed_t = 0o010003;
pub const B460800: ::speed_t = 0o010004;
pub const B500000: ::speed_t = 0o010005;
pub const B576000: ::speed_t = 0o010006;
pub const B921600: ::speed_t = 0o010007;
pub const B1000000: ::speed_t = 0o010010;
pub const B1152000: ::speed_t = 0o010011;
pub const B1500000: ::speed_t = 0o010012;
pub const B2000000: ::speed_t = 0o010013;
pub const B2500000: ::speed_t = 0o010014;
pub const B3000000: ::speed_t = 0o010015;
pub const B3500000: ::speed_t = 0o010016;
pub const B4000000: ::speed_t = 0o010017;

pub const VEOL: usize = 11;
pub const VEOL2: usize = 16;
pub const VMIN: usize = 6;
pub const IEXTEN: ::tcflag_t = 0x00008000;
pub const TOSTOP: ::tcflag_t = 0x00000100;
pub const FLUSHO: ::tcflag_t = 0x00001000;
pub const EXTPROC: ::tcflag_t = 0x00010000;
pub const TCGETS: ::c_ulong = 0x5401;
pub const TCSETS: ::c_ulong = 0x5402;
pub const TCSETSW: ::c_ulong = 0x5403;
pub const TCSETSF: ::c_ulong = 0x5404;
pub const TCGETA: ::c_ulong = 0x5405;
pub const TCSETA: ::c_ulong = 0x5406;
pub const TCSETAW: ::c_ulong = 0x5407;
pub const TCSETAF: ::c_ulong = 0x5408;
pub const TCSBRK: ::c_ulong = 0x5409;
pub const TCXONC: ::c_ulong = 0x540A;
pub const TCFLSH: ::c_ulong = 0x540B;
pub const TIOCINQ: ::c_ulong = 0x541B;
pub const TIOCGPGRP: ::c_ulong = 0x540F;
pub const TIOCSPGRP: ::c_ulong = 0x5410;
pub const TIOCOUTQ: ::c_ulong = 0x5411;
pub const TIOCGWINSZ: ::c_ulong = 0x5413;
pub const TIOCSWINSZ: ::c_ulong = 0x5414;
pub const FIONREAD: ::c_ulong = 0x541B;

// Syscall table
pub const SYS_restart_syscall: ::c_long = 0;
pub const SYS_exit: ::c_long = 1;
pub const SYS_fork: ::c_long = 2;
pub const SYS_read: ::c_long = 3;
pub const SYS_write: ::c_long = 4;
pub const SYS_open: ::c_long = 5;
pub const SYS_close: ::c_long = 6;
pub const SYS_creat: ::c_long = 8;
pub const SYS_link: ::c_long = 9;
pub const SYS_unlink: ::c_long = 10;
pub const SYS_execve: ::c_long = 11;
pub const SYS_chdir: ::c_long = 12;
pub const SYS_mknod: ::c_long = 14;
pub const SYS_chmod: ::c_long = 15;
pub const SYS_lchown: ::c_long = 16;
pub const SYS_lseek: ::c_long = 19;
pub const SYS_getpid: ::c_long = 20;
pub const SYS_mount: ::c_long = 21;
pub const SYS_setuid: ::c_long = 23;
pub const SYS_getuid: ::c_long = 24;
pub const SYS_ptrace: ::c_long = 26;
pub const SYS_pause: ::c_long = 29;
pub const SYS_access: ::c_long = 33;
pub const SYS_nice: ::c_long = 34;
pub const SYS_sync: ::c_long = 36;
pub const SYS_kill: ::c_long = 37;
pub const SYS_rename: ::c_long = 38;
pub const SYS_mkdir: ::c_long = 39;
pub const SYS_rmdir: ::c_long = 40;
pub const SYS_dup: ::c_long = 41;
pub const SYS_pipe: ::c_long = 42;
pub const SYS_times: ::c_long = 43;
pub const SYS_brk: ::c_long = 45;
pub const SYS_setgid: ::c_long = 46;
pub const SYS_getgid: ::c_long = 47;
pub const SYS_geteuid: ::c_long = 49;
pub const SYS_getegid: ::c_long = 50;
pub const SYS_acct: ::c_long = 51;
pub const SYS_umount2: ::c_long = 52;
pub const SYS_ioctl: ::c_long = 54;
pub const SYS_fcntl: ::c_long = 55;
pub const SYS_setpgid: ::c_long = 57;
pub const SYS_umask: ::c_long = 60;
pub const SYS_chroot: ::c_long = 61;
pub const SYS_ustat: ::c_long = 62;
pub const SYS_dup2: ::c_long = 63;
pub const SYS_getppid: ::c_long = 64;
pub const SYS_getpgrp: ::c_long = 65;
pub const SYS_setsid: ::c_long = 66;
pub const SYS_sigaction: ::c_long = 67;
pub const SYS_setreuid: ::c_long = 70;
pub const SYS_setregid: ::c_long = 71;
pub const SYS_sigsuspend: ::c_long = 72;
pub const SYS_sigpending: ::c_long = 73;
pub const SYS_sethostname: ::c_long = 74;
pub const SYS_setrlimit: ::c_long = 75;
pub const SYS_getrusage: ::c_long = 77;
pub const SYS_gettimeofday: ::c_long = 78;
pub const SYS_settimeofday: ::c_long = 79;
pub const SYS_getgroups: ::c_long = 80;
pub const SYS_setgroups: ::c_long = 81;
pub const SYS_symlink: ::c_long = 83;
pub const SYS_readlink: ::c_long = 85;
pub const SYS_uselib: ::c_long = 86;
pub const SYS_swapon: ::c_long = 87;
pub const SYS_reboot: ::c_long = 88;
pub const SYS_munmap: ::c_long = 91;
pub const SYS_truncate: ::c_long = 92;
pub const SYS_ftruncate: ::c_long = 93;
pub const SYS_fchmod: ::c_long = 94;
pub const SYS_fchown: ::c_long = 95;
pub const SYS_getpriority: ::c_long = 96;
pub const SYS_setpriority: ::c_long = 97;
pub const SYS_statfs: ::c_long = 99;
pub const SYS_fstatfs: ::c_long = 100;
pub const SYS_syslog: ::c_long = 103;
pub const SYS_setitimer: ::c_long = 104;
pub const SYS_getitimer: ::c_long = 105;
pub const SYS_stat: ::c_long = 106;
pub const SYS_lstat: ::c_long = 107;
pub const SYS_fstat: ::c_long = 108;
pub const SYS_vhangup: ::c_long = 111;
pub const SYS_wait4: ::c_long = 114;
pub const SYS_swapoff: ::c_long = 115;
pub const SYS_sysinfo: ::c_long = 116;
pub const SYS_fsync: ::c_long = 118;
pub const SYS_sigreturn: ::c_long = 119;
pub const SYS_clone: ::c_long = 120;
pub const SYS_setdomainname: ::c_long = 121;
pub const SYS_uname: ::c_long = 122;
pub const SYS_adjtimex: ::c_long = 124;
pub const SYS_mprotect: ::c_long = 125;
pub const SYS_sigprocmask: ::c_long = 126;
pub const SYS_init_module: ::c_long = 128;
pub const SYS_delete_module: ::c_long = 129;
pub const SYS_quotactl: ::c_long = 131;
pub const SYS_getpgid: ::c_long = 132;
pub const SYS_fchdir: ::c_long = 133;
pub const SYS_bdflush: ::c_long = 134;
pub const SYS_sysfs: ::c_long = 135;
pub const SYS_personality: ::c_long = 136;
pub const SYS_setfsuid: ::c_long = 138;
pub const SYS_setfsgid: ::c_long = 139;
pub const SYS__llseek: ::c_long = 140;
pub const SYS_getdents: ::c_long = 141;
pub const SYS__newselect: ::c_long = 142;
pub const SYS_flock: ::c_long = 143;
pub const SYS_msync: ::c_long = 144;
pub const SYS_readv: ::c_long = 145;
pub const SYS_writev: ::c_long = 146;
pub const SYS_getsid: ::c_long = 147;
pub const SYS_fdatasync: ::c_long = 148;
pub const SYS__sysctl: ::c_long = 149;
pub const SYS_mlock: ::c_long = 150;
pub const SYS_munlock: ::c_long = 151;
pub const SYS_mlockall: ::c_long = 152;
pub const SYS_munlockall: ::c_long = 153;
pub const SYS_sched_setparam: ::c_long = 154;
pub const SYS_sched_getparam: ::c_long = 155;
pub const SYS_sched_setscheduler: ::c_long = 156;
pub const SYS_sched_getscheduler: ::c_long = 157;
pub const SYS_sched_yield: ::c_long = 158;
pub const SYS_sched_get_priority_max: ::c_long = 159;
pub const SYS_sched_get_priority_min: ::c_long = 160;
pub const SYS_sched_rr_get_interval: ::c_long = 161;
pub const SYS_nanosleep: ::c_long = 162;
pub const SYS_mremap: ::c_long = 163;
pub const SYS_setresuid: ::c_long = 164;
pub const SYS_getresuid: ::c_long = 165;
pub const SYS_poll: ::c_long = 168;
pub const SYS_nfsservctl: ::c_long = 169;
pub const SYS_setresgid: ::c_long = 170;
pub const SYS_getresgid: ::c_long = 171;
pub const SYS_prctl: ::c_long = 172;
pub const SYS_rt_sigreturn: ::c_long = 173;
pub const SYS_rt_sigaction: ::c_long = 174;
pub const SYS_rt_sigprocmask: ::c_long = 175;
pub const SYS_rt_sigpending: ::c_long = 176;
pub const SYS_rt_sigtimedwait: ::c_long = 177;
pub const SYS_rt_sigqueueinfo: ::c_long = 178;
pub const SYS_rt_sigsuspend: ::c_long = 179;
pub const SYS_pread64: ::c_long = 180;
pub const SYS_pwrite64: ::c_long = 181;
pub const SYS_chown: ::c_long = 182;
pub const SYS_getcwd: ::c_long = 183;
pub const SYS_capget: ::c_long = 184;
pub const SYS_capset: ::c_long = 185;
pub const SYS_sigaltstack: ::c_long = 186;
pub const SYS_sendfile: ::c_long = 187;
pub const SYS_vfork: ::c_long = 190;
pub const SYS_ugetrlimit: ::c_long = 191;
pub const SYS_mmap2: ::c_long = 192;
pub const SYS_truncate64: ::c_long = 193;
pub const SYS_ftruncate64: ::c_long = 194;
pub const SYS_stat64: ::c_long = 195;
pub const SYS_lstat64: ::c_long = 196;
pub const SYS_fstat64: ::c_long = 197;
pub const SYS_lchown32: ::c_long = 198;
pub const SYS_getuid32: ::c_long = 199;
pub const SYS_getgid32: ::c_long = 200;
pub const SYS_geteuid32: ::c_long = 201;
pub const SYS_getegid32: ::c_long = 202;
pub const SYS_setreuid32: ::c_long = 203;
pub const SYS_setregid32: ::c_long = 204;
pub const SYS_getgroups32: ::c_long = 205;
pub const SYS_setgroups32: ::c_long = 206;
pub const SYS_fchown32: ::c_long = 207;
pub const SYS_setresuid32: ::c_long = 208;
pub const SYS_getresuid32: ::c_long = 209;
pub const SYS_setresgid32: ::c_long = 210;
pub const SYS_getresgid32: ::c_long = 211;
pub const SYS_chown32: ::c_long = 212;
pub const SYS_setuid32: ::c_long = 213;
pub const SYS_setgid32: ::c_long = 214;
pub const SYS_setfsuid32: ::c_long = 215;
pub const SYS_setfsgid32: ::c_long = 216;
pub const SYS_getdents64: ::c_long = 217;
pub const SYS_pivot_root: ::c_long = 218;
pub const SYS_mincore: ::c_long = 219;
pub const SYS_madvise: ::c_long = 220;
pub const SYS_fcntl64: ::c_long = 221;
pub const SYS_gettid: ::c_long = 224;
pub const SYS_readahead: ::c_long = 225;
pub const SYS_setxattr: ::c_long = 226;
pub const SYS_lsetxattr: ::c_long = 227;
pub const SYS_fsetxattr: ::c_long = 228;
pub const SYS_getxattr: ::c_long = 229;
pub const SYS_lgetxattr: ::c_long = 230;
pub const SYS_fgetxattr: ::c_long = 231;
pub const SYS_listxattr: ::c_long = 232;
pub const SYS_llistxattr: ::c_long = 233;
pub const SYS_flistxattr: ::c_long = 234;
pub const SYS_removexattr: ::c_long = 235;
pub const SYS_lremovexattr: ::c_long = 236;
pub const SYS_fremovexattr: ::c_long = 237;
pub const SYS_tkill: ::c_long = 238;
pub const SYS_sendfile64: ::c_long = 239;
pub const SYS_futex: ::c_long = 240;
pub const SYS_sched_setaffinity: ::c_long = 241;
pub const SYS_sched_getaffinity: ::c_long = 242;
pub const SYS_io_setup: ::c_long = 243;
pub const SYS_io_destroy: ::c_long = 244;
pub const SYS_io_getevents: ::c_long = 245;
pub const SYS_io_submit: ::c_long = 246;
pub const SYS_io_cancel: ::c_long = 247;
pub const SYS_exit_group: ::c_long = 248;
pub const SYS_lookup_dcookie: ::c_long = 249;
pub const SYS_epoll_create: ::c_long = 250;
pub const SYS_epoll_ctl: ::c_long = 251;
pub const SYS_epoll_wait: ::c_long = 252;
pub const SYS_remap_file_pages: ::c_long = 253;
pub const SYS_set_tid_address: ::c_long = 256;
pub const SYS_timer_create: ::c_long = 257;
pub const SYS_timer_settime: ::c_long = 258;
pub const SYS_timer_gettime: ::c_long = 259;
pub const SYS_timer_getoverrun: ::c_long = 260;
pub const SYS_timer_delete: ::c_long = 261;
pub const SYS_clock_settime: ::c_long = 262;
pub const SYS_clock_gettime: ::c_long = 263;
pub const SYS_clock_getres: ::c_long = 264;
pub const SYS_clock_nanosleep: ::c_long = 265;
pub const SYS_statfs64: ::c_long = 266;
pub const SYS_fstatfs64: ::c_long = 267;
pub const SYS_tgkill: ::c_long = 268;
pub const SYS_utimes: ::c_long = 269;
pub const SYS_arm_fadvise64_64: ::c_long = 270;
pub const SYS_pciconfig_iobase: ::c_long = 271;
pub const SYS_pciconfig_read: ::c_long = 272;
pub const SYS_pciconfig_write: ::c_long = 273;
pub const SYS_mq_open: ::c_long = 274;
pub const SYS_mq_unlink: ::c_long = 275;
pub const SYS_mq_timedsend: ::c_long = 276;
pub const SYS_mq_timedreceive: ::c_long = 277;
pub const SYS_mq_notify: ::c_long = 278;
pub const SYS_mq_getsetattr: ::c_long = 279;
pub const SYS_waitid: ::c_long = 280;
pub const SYS_socket: ::c_long = 281;
pub const SYS_bind: ::c_long = 282;
pub const SYS_connect: ::c_long = 283;
pub const SYS_listen: ::c_long = 284;
pub const SYS_accept: ::c_long = 285;
pub const SYS_getsockname: ::c_long = 286;
pub const SYS_getpeername: ::c_long = 287;
pub const SYS_socketpair: ::c_long = 288;
pub const SYS_send: ::c_long = 289;
pub const SYS_sendto: ::c_long = 290;
pub const SYS_recv: ::c_long = 291;
pub const SYS_recvfrom: ::c_long = 292;
pub const SYS_shutdown: ::c_long = 293;
pub const SYS_setsockopt: ::c_long = 294;
pub const SYS_getsockopt: ::c_long = 295;
pub const SYS_sendmsg: ::c_long = 296;
pub const SYS_recvmsg: ::c_long = 297;
pub const SYS_semop: ::c_long = 298;
pub const SYS_semget: ::c_long = 299;
pub const SYS_semctl: ::c_long = 300;
pub const SYS_msgsnd: ::c_long = 301;
pub const SYS_msgrcv: ::c_long = 302;
pub const SYS_msgget: ::c_long = 303;
pub const SYS_msgctl: ::c_long = 304;
pub const SYS_shmat: ::c_long = 305;
pub const SYS_shmdt: ::c_long = 306;
pub const SYS_shmget: ::c_long = 307;
pub const SYS_shmctl: ::c_long = 308;
pub const SYS_add_key: ::c_long = 309;
pub const SYS_request_key: ::c_long = 310;
pub const SYS_keyctl: ::c_long = 311;
pub const SYS_semtimedop: ::c_long = 312;
pub const SYS_vserver: ::c_long = 313;
pub const SYS_ioprio_set: ::c_long = 314;
pub const SYS_ioprio_get: ::c_long = 315;
pub const SYS_inotify_init: ::c_long = 316;
pub const SYS_inotify_add_watch: ::c_long = 317;
pub const SYS_inotify_rm_watch: ::c_long = 318;
pub const SYS_mbind: ::c_long = 319;
pub const SYS_get_mempolicy: ::c_long = 320;
pub const SYS_set_mempolicy: ::c_long = 321;
pub const SYS_openat: ::c_long = 322;
pub const SYS_mkdirat: ::c_long = 323;
pub const SYS_mknodat: ::c_long = 324;
pub const SYS_fchownat: ::c_long = 325;
pub const SYS_futimesat: ::c_long = 326;
pub const SYS_fstatat64: ::c_long = 327;
pub const SYS_unlinkat: ::c_long = 328;
pub const SYS_renameat: ::c_long = 329;
pub const SYS_linkat: ::c_long = 330;
pub const SYS_symlinkat: ::c_long = 331;
pub const SYS_readlinkat: ::c_long = 332;
pub const SYS_fchmodat: ::c_long = 333;
pub const SYS_faccessat: ::c_long = 334;
pub const SYS_pselect6: ::c_long = 335;
pub const SYS_ppoll: ::c_long = 336;
pub const SYS_unshare: ::c_long = 337;
pub const SYS_set_robust_list: ::c_long = 338;
pub const SYS_get_robust_list: ::c_long = 339;
pub const SYS_splice: ::c_long = 340;
pub const SYS_arm_sync_file_range: ::c_long = 341;
pub const SYS_tee: ::c_long = 342;
pub const SYS_vmsplice: ::c_long = 343;
pub const SYS_move_pages: ::c_long = 344;
pub const SYS_getcpu: ::c_long = 345;
pub const SYS_epoll_pwait: ::c_long = 346;
pub const SYS_kexec_load: ::c_long = 347;
pub const SYS_utimensat: ::c_long = 348;
pub const SYS_signalfd: ::c_long = 349;
pub const SYS_timerfd_create: ::c_long = 350;
pub const SYS_eventfd: ::c_long = 351;
pub const SYS_fallocate: ::c_long = 352;
pub const SYS_timerfd_settime: ::c_long = 353;
pub const SYS_timerfd_gettime: ::c_long = 354;
pub const SYS_signalfd4: ::c_long = 355;
pub const SYS_eventfd2: ::c_long = 356;
pub const SYS_epoll_create1: ::c_long = 357;
pub const SYS_dup3: ::c_long = 358;
pub const SYS_pipe2: ::c_long = 359;
pub const SYS_inotify_init1: ::c_long = 360;
pub const SYS_preadv: ::c_long = 361;
pub const SYS_pwritev: ::c_long = 362;
pub const SYS_rt_tgsigqueueinfo: ::c_long = 363;
pub const SYS_perf_event_open: ::c_long = 364;
pub const SYS_recvmmsg: ::c_long = 365;
pub const SYS_accept4: ::c_long = 366;
pub const SYS_fanotify_init: ::c_long = 367;
pub const SYS_fanotify_mark: ::c_long = 368;
pub const SYS_prlimit64: ::c_long = 369;
pub const SYS_name_to_handle_at: ::c_long = 370;
pub const SYS_open_by_handle_at: ::c_long = 371;
pub const SYS_clock_adjtime: ::c_long = 372;
pub const SYS_syncfs: ::c_long = 373;
pub const SYS_sendmmsg: ::c_long = 374;
pub const SYS_setns: ::c_long = 375;
pub const SYS_process_vm_readv: ::c_long = 376;
pub const SYS_process_vm_writev: ::c_long = 377;
pub const SYS_kcmp: ::c_long = 378;
pub const SYS_finit_module: ::c_long = 379;
pub const SYS_sched_setattr: ::c_long = 380;
pub const SYS_sched_getattr: ::c_long = 381;
pub const SYS_renameat2: ::c_long = 382;
pub const SYS_seccomp: ::c_long = 383;
pub const SYS_getrandom: ::c_long = 384;
pub const SYS_memfd_create: ::c_long = 385;
pub const SYS_bpf: ::c_long = 386;
pub const SYS_execveat: ::c_long = 387;
pub const SYS_userfaultfd: ::c_long = 388;
pub const SYS_membarrier: ::c_long = 389;
pub const SYS_mlock2: ::c_long = 390;
pub const SYS_copy_file_range: ::c_long = 391;
pub const SYS_preadv2: ::c_long = 392;
pub const SYS_pwritev2: ::c_long = 393;
pub const SYS_pkey_mprotect: ::c_long = 394;
pub const SYS_pkey_alloc: ::c_long = 395;
pub const SYS_pkey_free: ::c_long = 396;



pub type c_long = i32;
pub type c_ulong = u32;
pub type clock_t = i32;
pub type time_t = i32;
pub type suseconds_t = i32;
pub type ino_t = u32;
pub type off_t = i32;
pub type blkcnt_t = i32;

pub type fsblkcnt_t = ::c_ulong;
pub type fsfilcnt_t = ::c_ulong;
pub type rlim_t = c_ulong;
pub type blksize_t = i32;
pub type nlink_t = u32;
pub type __u64 = ::c_ulonglong;

s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        __pad1: ::c_short,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        __pad2: ::c_short,
        pub st_size: ::off_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        __unused4: ::c_long,
        __unused5: ::c_long,
    }

    pub struct pthread_attr_t {
        __size: [u32; 9]
    }

    pub struct sigset_t {
        __val: [::c_ulong; 2],
    }

    pub struct sysinfo {
        pub uptime: ::c_long,
        pub loads: [::c_ulong; 3],
        pub totalram: ::c_ulong,
        pub freeram: ::c_ulong,
        pub sharedram: ::c_ulong,
        pub bufferram: ::c_ulong,
        pub totalswap: ::c_ulong,
        pub freeswap: ::c_ulong,
        pub procs: ::c_ushort,
        pub pad: ::c_ushort,
        pub totalhigh: ::c_ulong,
        pub freehigh: ::c_ulong,
        pub mem_unit: ::c_uint,
        pub _f: [::c_char; 8],
    }
}

pub type pthread_t = c_ulong;

pub type __u8 = ::c_uchar;
pub type __u16 = ::c_ushort;
pub type __s16 = ::c_short;
pub type __u32 = ::c_uint;
pub type __s32 = ::c_int;

pub type Elf32_Half = u16;
pub type Elf32_Word = u32;
pub type Elf32_Off = u32;
pub type Elf32_Addr = u32;

pub type Elf64_Half = u16;
pub type Elf64_Word = u32;
pub type Elf64_Off = u64;
pub type Elf64_Addr = u64;
pub type Elf64_Xword = u64;

pub enum fpos64_t {} // TODO: fill this out with a struct

s! {
    pub struct itimerspec {
        pub it_interval: ::timespec,
        pub it_value: ::timespec,
    }

    pub struct sembuf {
        pub sem_num: ::c_ushort,
        pub sem_op: ::c_short,
        pub sem_flg: ::c_short,
    }

    pub struct input_event {
        pub time: ::timeval,
        pub type_: ::__u16,
        pub code: ::__u16,
        pub value: ::__s32,
    }

    pub struct input_id {
        pub bustype: ::__u16,
        pub vendor: ::__u16,
        pub product: ::__u16,
        pub version: ::__u16,
    }

    pub struct input_absinfo {
        pub value: ::__s32,
        pub minimum: ::__s32,
        pub maximum: ::__s32,
        pub fuzz: ::__s32,
        pub flat: ::__s32,
        pub resolution: ::__s32,
    }

    pub struct input_keymap_entry {
        pub flags: ::__u8,
        pub len: ::__u8,
        pub index: ::__u16,
        pub keycode: ::__u32,
        pub scancode: [::__u8; 32],
    }

    pub struct ff_replay {
        pub length: ::__u16,
        pub delay: ::__u16,
    }

    pub struct ff_trigger {
        pub button: ::__u16,
        pub interval: ::__u16,
    }

    pub struct ff_envelope {
        pub attack_length: ::__u16,
        pub attack_level: ::__u16,
        pub fade_length: ::__u16,
        pub fade_level: ::__u16,
    }

    pub struct ff_constant_effect {
        pub level: ::__s16,
        pub envelope: ff_envelope,
    }

    pub struct ff_ramp_effect {
        pub start_level: ::__s16,
        pub end_level: ::__s16,
        pub envelope: ff_envelope,
    }

    pub struct ff_condition_effect {
        pub right_saturation: ::__u16,
        pub left_saturation: ::__u16,

        pub right_coeff: ::__s16,
        pub left_coeff: ::__s16,

        pub deadband: ::__u16,
        pub center: ::__s16,
    }

    pub struct ff_periodic_effect {
        pub waveform: ::__u16,
        pub period: ::__u16,
        pub magnitude: ::__s16,
        pub offset: ::__s16,
        pub phase: ::__u16,

        pub envelope: ff_envelope,

        pub custom_len: ::__u32,
        pub custom_data: *mut ::__s16,
    }

    pub struct ff_rumble_effect {
        pub strong_magnitude: ::__u16,
        pub weak_magnitude: ::__u16,
    }

    pub struct ff_effect {
        pub type_: ::__u16,
        pub id: ::__s16,
        pub direction: ::__u16,
        pub trigger: ff_trigger,
        pub replay: ff_replay,
        // FIXME this is actually a union
        #[cfg(target_pointer_width = "64")]
        pub u: [u64; 4],
        #[cfg(target_pointer_width = "32")]
        pub u: [u32; 7],
    }

    pub struct Elf32_Phdr {
        pub p_type: Elf32_Word,
        pub p_offset: Elf32_Off,
        pub p_vaddr: Elf32_Addr,
        pub p_paddr: Elf32_Addr,
        pub p_filesz: Elf32_Word,
        pub p_memsz: Elf32_Word,
        pub p_flags: Elf32_Word,
        pub p_align: Elf32_Word,
    }

    pub struct Elf64_Phdr {
        pub p_type: Elf64_Word,
        pub p_flags: Elf64_Word,
        pub p_offset: Elf64_Off,
        pub p_vaddr: Elf64_Addr,
        pub p_paddr: Elf64_Addr,
        pub p_filesz: Elf64_Xword,
        pub p_memsz: Elf64_Xword,
        pub p_align: Elf64_Xword,
    }

    pub struct ucred {
        pub pid: ::pid_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
    }

    pub struct mntent {
        pub mnt_fsname: *mut ::c_char,
        pub mnt_dir: *mut ::c_char,
        pub mnt_type: *mut ::c_char,
        pub mnt_opts: *mut ::c_char,
        pub mnt_freq: ::c_int,
        pub mnt_passno: ::c_int,
    }

    pub struct posix_spawn_file_actions_t {
        __allocated: ::c_int,
        __used: ::c_int,
        __actions: *mut ::c_int,
        __pad: [::c_int; 16],
    }

    pub struct posix_spawnattr_t {
        __flags: ::c_short,
        __pgrp: ::pid_t,
        __sd: ::sigset_t,
        __ss: ::sigset_t,
        #[cfg(target_env = "musl")]
        __prio: ::c_int,
        #[cfg(not(target_env = "musl"))]
        __sp: ::sched_param,
        __policy: ::c_int,
        __pad: [::c_int; 16],
    }

    pub struct genlmsghdr {
        pub cmd: u8,
        pub version: u8,
        pub reserved: u16,
    }

    pub struct in6_pktinfo {
        pub ipi6_addr: ::in6_addr,
        pub ipi6_ifindex: ::c_uint,
    }

    pub struct arpd_request {
        pub req: ::c_ushort,
        pub ip: u32,
        pub dev: ::c_ulong,
        pub stamp: ::c_ulong,
        pub updated: ::c_ulong,
        pub ha: [::c_uchar; ::MAX_ADDR_LEN],
    }
}

pub const RUSAGE_CHILDREN: ::c_int = -1;

// Weird endianness issue - is this needed?
// pub const YESEXPR: ::nl_item = 0x50000;

pub const _PC_SYNC_IO: ::c_int = 9;
pub const _PC_ASYNC_IO: ::c_int = 10;
pub const _PC_PRIO_IO: ::c_int = 11;
pub const _PC_SOCK_MAXBUF: ::c_int = 12;
pub const _PC_FILESIZEBITS: ::c_int = 13;
pub const _PC_REC_INCR_XFER_SIZE: ::c_int = 14;
pub const _PC_REC_MAX_XFER_SIZE: ::c_int = 15;
pub const _PC_REC_MIN_XFER_SIZE: ::c_int = 16;
pub const _PC_REC_XFER_ALIGN: ::c_int = 17;
pub const _PC_ALLOC_SIZE_MIN: ::c_int = 18;
pub const _PC_SYMLINK_MAX: ::c_int = 19;
pub const _PC_2_SYMLINKS: ::c_int = 20;

// linux/if_tun.h
pub const IFF_TUN: ::c_short = 0x0001;
pub const IFF_TAP: ::c_short = 0x0002;
pub const IFF_NO_PI: ::c_short = 0x1000;
// Read queue size
pub const TUN_READQ_SIZE: ::c_short = 500;
// TUN device type flags: deprecated. Use IFF_TUN/IFF_TAP instead.
pub const TUN_TUN_DEV: ::c_short   = ::IFF_TUN;
pub const TUN_TAP_DEV: ::c_short   = ::IFF_TAP;
pub const TUN_TYPE_MASK: ::c_short = 0x000f;
// This flag has no real effect
pub const IFF_ONE_QUEUE: ::c_short    = 0x2000;
pub const IFF_VNET_HDR: ::c_short     = 0x4000;
pub const IFF_TUN_EXCL: ::c_short     = 0x8000;
pub const IFF_MULTI_QUEUE: ::c_short  = 0x0100;
pub const IFF_ATTACH_QUEUE: ::c_short = 0x0200;
pub const IFF_DETACH_QUEUE: ::c_short = 0x0400;
// read-only flag
pub const IFF_PERSIST: ::c_short  = 0x0800;
pub const IFF_NOFILTER: ::c_short = 0x1000;

pub const PTHREAD_PROCESS_PRIVATE: ::c_int = 0;
pub const PTHREAD_PROCESS_SHARED: ::c_int = 1;

// netinet/in.h
// NOTE: These are in addition to the constants defined in src/unix/mod.rs

// System V IPC
pub const QFMT_VFS_V1: ::c_int = 4;

pub const SECCOMP_MODE_DISABLED: ::c_uint = 0;
pub const SECCOMP_MODE_STRICT: ::c_uint = 1;
pub const SECCOMP_MODE_FILTER: ::c_uint = 2;

pub const ITIMER_REAL: ::c_int = 0;
pub const ITIMER_VIRTUAL: ::c_int = 1;
pub const ITIMER_PROF: ::c_int = 2;

pub const TFD_CLOEXEC: ::c_int = O_CLOEXEC;
pub const TFD_NONBLOCK: ::c_int = O_NONBLOCK;
pub const TFD_TIMER_ABSTIME: ::c_int = 1;

pub const _POSIX_VDISABLE: ::cc_t = 0;

pub const FALLOC_FL_KEEP_SIZE: ::c_int = 0x01;
pub const FALLOC_FL_PUNCH_HOLE: ::c_int = 0x02;
pub const FALLOC_FL_COLLAPSE_RANGE: ::c_int = 0x08;
pub const FALLOC_FL_ZERO_RANGE: ::c_int = 0x10;
pub const FALLOC_FL_INSERT_RANGE: ::c_int = 0x20;
pub const FALLOC_FL_UNSHARE_RANGE: ::c_int = 0x40;

// On Linux, libc doesn't define this constant, libattr does instead.
// We still define it for Linux as it's defined by libc on other platforms,
// and it's mentioned in the man pages for getxattr and setxattr.
pub const ENOATTR: ::c_int = ::ENODATA;

pub const SO_ORIGINAL_DST: ::c_int = 80;
pub const IUTF8: ::tcflag_t = 0x00004000;
pub const CMSPAR: ::tcflag_t = 0o10000000000;

pub const MFD_CLOEXEC: ::c_uint = 0x0001;
pub const MFD_ALLOW_SEALING: ::c_uint = 0x0002;

// these are used in the p_type field of Elf32_Phdr and Elf64_Phdr, which has
// the type Elf32Word and Elf64Word respectively. Luckily, both of those are u32
// so we can use that type here to avoid having to cast.
pub const PT_NULL: u32 = 0;
pub const PT_LOAD: u32 = 1;
pub const PT_DYNAMIC: u32 = 2;
pub const PT_INTERP: u32 = 3;
pub const PT_NOTE: u32 = 4;
pub const PT_SHLIB: u32 = 5;
pub const PT_PHDR: u32 = 6;
pub const PT_TLS: u32 = 7;
pub const PT_NUM: u32 = 8;
pub const PT_LOOS: u32 = 0x60000000;
pub const PT_GNU_EH_FRAME: u32 = 0x6474e550;
pub const PT_GNU_STACK: u32 = 0x6474e551;
pub const PT_GNU_RELRO: u32 = 0x6474e552;

// linux/if_ether.h
pub const ETH_ALEN: ::c_int = 6;
pub const ETH_HLEN: ::c_int = 14;
pub const ETH_ZLEN: ::c_int = 60;
pub const ETH_DATA_LEN: ::c_int = 1500;
pub const ETH_FRAME_LEN: ::c_int = 1514;
pub const ETH_FCS_LEN: ::c_int = 4;

// These are the defined Ethernet Protocol ID's.
pub const ETH_P_LOOP: ::c_int = 0x0060;
pub const ETH_P_PUP: ::c_int = 0x0200;
pub const ETH_P_PUPAT: ::c_int = 0x0201;
pub const ETH_P_IP: ::c_int = 0x0800;
pub const ETH_P_X25: ::c_int = 0x0805;
pub const ETH_P_ARP: ::c_int = 0x0806;
pub const ETH_P_BPQ: ::c_int = 0x08FF;
pub const ETH_P_IEEEPUP: ::c_int = 0x0a00;
pub const ETH_P_IEEEPUPAT: ::c_int = 0x0a01;
pub const ETH_P_BATMAN: ::c_int = 0x4305;
pub const ETH_P_DEC: ::c_int = 0x6000;
pub const ETH_P_DNA_DL: ::c_int = 0x6001;
pub const ETH_P_DNA_RC: ::c_int = 0x6002;
pub const ETH_P_DNA_RT: ::c_int = 0x6003;
pub const ETH_P_LAT: ::c_int = 0x6004;
pub const ETH_P_DIAG: ::c_int = 0x6005;
pub const ETH_P_CUST: ::c_int = 0x6006;
pub const ETH_P_SCA: ::c_int = 0x6007;
pub const ETH_P_TEB: ::c_int = 0x6558;
pub const ETH_P_RARP: ::c_int = 0x8035;
pub const ETH_P_ATALK: ::c_int = 0x809B;
pub const ETH_P_AARP: ::c_int = 0x80F3;
pub const ETH_P_8021Q: ::c_int = 0x8100;
pub const ETH_P_IPX: ::c_int = 0x8137;
pub const ETH_P_IPV6: ::c_int = 0x86DD;
pub const ETH_P_PAUSE: ::c_int = 0x8808;
pub const ETH_P_SLOW: ::c_int = 0x8809;
pub const ETH_P_WCCP: ::c_int = 0x883E;
pub const ETH_P_MPLS_UC: ::c_int = 0x8847;
pub const ETH_P_MPLS_MC: ::c_int = 0x8848;
pub const ETH_P_ATMMPOA: ::c_int = 0x884c;
pub const ETH_P_PPP_DISC: ::c_int = 0x8863;
pub const ETH_P_PPP_SES: ::c_int = 0x8864;
pub const ETH_P_LINK_CTL: ::c_int = 0x886c;
pub const ETH_P_ATMFATE: ::c_int = 0x8884;
pub const ETH_P_PAE: ::c_int = 0x888E;
pub const ETH_P_AOE: ::c_int = 0x88A2;
pub const ETH_P_8021AD: ::c_int = 0x88A8;
pub const ETH_P_802_EX1: ::c_int = 0x88B5;
pub const ETH_P_TIPC: ::c_int = 0x88CA;
pub const ETH_P_8021AH: ::c_int = 0x88E7;
pub const ETH_P_MVRP: ::c_int = 0x88F5;
pub const ETH_P_1588: ::c_int = 0x88F7;
pub const ETH_P_PRP: ::c_int = 0x88FB;
pub const ETH_P_FCOE: ::c_int = 0x8906;
pub const ETH_P_TDLS: ::c_int = 0x890D;
pub const ETH_P_FIP: ::c_int = 0x8914;
pub const ETH_P_QINQ1: ::c_int = 0x9100;
pub const ETH_P_QINQ2: ::c_int = 0x9200;
pub const ETH_P_QINQ3: ::c_int = 0x9300;
pub const ETH_P_EDSA: ::c_int = 0xDADA;
pub const ETH_P_AF_IUCV: ::c_int = 0xFBFB;

pub const ETH_P_802_3_MIN: ::c_int = 0x0600;

// Non DIX types. Won't clash for 1500 types.
pub const ETH_P_802_3: ::c_int = 0x0001;
pub const ETH_P_AX25: ::c_int = 0x0002;
pub const ETH_P_ALL: ::c_int = 0x0003;
pub const ETH_P_802_2: ::c_int = 0x0004;
pub const ETH_P_SNAP: ::c_int = 0x0005;
pub const ETH_P_DDCMP: ::c_int = 0x0006;
pub const ETH_P_WAN_PPP: ::c_int = 0x0007;
pub const ETH_P_PPP_MP: ::c_int = 0x0008;
pub const ETH_P_LOCALTALK: ::c_int = 0x0009;
pub const ETH_P_CANFD: ::c_int = 0x000D;
pub const ETH_P_PPPTALK: ::c_int = 0x0010;
pub const ETH_P_TR_802_2: ::c_int = 0x0011;
pub const ETH_P_MOBITEX: ::c_int = 0x0015;
pub const ETH_P_CONTROL: ::c_int = 0x0016;
pub const ETH_P_IRDA: ::c_int = 0x0017;
pub const ETH_P_ECONET: ::c_int = 0x0018;
pub const ETH_P_HDLC: ::c_int = 0x0019;
pub const ETH_P_ARCNET: ::c_int = 0x001A;
pub const ETH_P_DSA: ::c_int = 0x001B;
pub const ETH_P_TRAILER: ::c_int = 0x001C;
pub const ETH_P_PHONET: ::c_int = 0x00F5;
pub const ETH_P_IEEE802154: ::c_int = 0x00F6;
pub const ETH_P_CAIF: ::c_int = 0x00F7;

pub const POSIX_SPAWN_RESETIDS: ::c_int = 0x01;
pub const POSIX_SPAWN_SETPGROUP: ::c_int = 0x02;
pub const POSIX_SPAWN_SETSIGDEF: ::c_int = 0x04;
pub const POSIX_SPAWN_SETSIGMASK: ::c_int = 0x08;
pub const POSIX_SPAWN_SETSCHEDPARAM: ::c_int = 0x10;
pub const POSIX_SPAWN_SETSCHEDULER: ::c_int = 0x20;

pub const NLMSG_NOOP: ::c_int = 0x1;
pub const NLMSG_ERROR: ::c_int = 0x2;
pub const NLMSG_DONE: ::c_int = 0x3;
pub const NLMSG_OVERRUN: ::c_int = 0x4;
pub const NLMSG_MIN_TYPE: ::c_int = 0x10;

pub const GENL_NAMSIZ: ::c_int = 16;

pub const GENL_MIN_ID: ::c_int = NLMSG_MIN_TYPE;
pub const GENL_MAX_ID: ::c_int = 1023;

pub const GENL_ADMIN_PERM: ::c_int = 0x01;
pub const GENL_CMD_CAP_DO: ::c_int = 0x02;
pub const GENL_CMD_CAP_DUMP: ::c_int = 0x04;
pub const GENL_CMD_CAP_HASPOL: ::c_int = 0x08;

pub const GENL_ID_CTRL: ::c_int = NLMSG_MIN_TYPE;

pub const CTRL_CMD_UNSPEC: ::c_int = 0;
pub const CTRL_CMD_NEWFAMILY: ::c_int = 1;
pub const CTRL_CMD_DELFAMILY: ::c_int = 2;
pub const CTRL_CMD_GETFAMILY: ::c_int = 3;
pub const CTRL_CMD_NEWOPS: ::c_int = 4;
pub const CTRL_CMD_DELOPS: ::c_int = 5;
pub const CTRL_CMD_GETOPS: ::c_int = 6;
pub const CTRL_CMD_NEWMCAST_GRP: ::c_int = 7;
pub const CTRL_CMD_DELMCAST_GRP: ::c_int = 8;
pub const CTRL_CMD_GETMCAST_GRP: ::c_int = 9;

pub const CTRL_ATTR_UNSPEC: ::c_int = 0;
pub const CTRL_ATTR_FAMILY_ID: ::c_int = 1;
pub const CTRL_ATTR_FAMILY_NAME: ::c_int = 2;
pub const CTRL_ATTR_VERSION: ::c_int = 3;
pub const CTRL_ATTR_HDRSIZE: ::c_int = 4;
pub const CTRL_ATTR_MAXATTR: ::c_int = 5;
pub const CTRL_ATTR_OPS: ::c_int = 6;
pub const CTRL_ATTR_MCAST_GROUPS: ::c_int = 7;

pub const CTRL_ATTR_OP_UNSPEC: ::c_int = 0;
pub const CTRL_ATTR_OP_ID: ::c_int = 1;
pub const CTRL_ATTR_OP_FLAGS: ::c_int = 2;

pub const CTRL_ATTR_MCAST_GRP_UNSPEC: ::c_int = 0;
pub const CTRL_ATTR_MCAST_GRP_NAME: ::c_int = 1;
pub const CTRL_ATTR_MCAST_GRP_ID: ::c_int = 2;

// linux/netfilter.h
pub const NF_DROP: ::c_int = 0;
pub const NF_ACCEPT: ::c_int =  1;
pub const NF_STOLEN: ::c_int =  2;
pub const NF_QUEUE: ::c_int =  3;
pub const NF_REPEAT: ::c_int =  4;
pub const NF_STOP: ::c_int =  5;
pub const NF_MAX_VERDICT: ::c_int = NF_STOP;

pub const NF_VERDICT_MASK: ::c_int = 0x000000ff;
pub const NF_VERDICT_FLAG_QUEUE_BYPASS: ::c_int = 0x00008000;

pub const NF_VERDICT_QMASK: ::c_int = 0xffff0000;
pub const NF_VERDICT_QBITS: ::c_int = 16;

pub const NF_VERDICT_BITS: ::c_int = 16;

pub const NF_INET_PRE_ROUTING: ::c_int = 0;
pub const NF_INET_LOCAL_IN: ::c_int = 1;
pub const NF_INET_FORWARD: ::c_int = 2;
pub const NF_INET_LOCAL_OUT: ::c_int = 3;
pub const NF_INET_POST_ROUTING: ::c_int = 4;
pub const NF_INET_NUMHOOKS: ::c_int = 5;

// Some NFPROTO are not compatible with musl and are defined in submodules.
pub const NFPROTO_UNSPEC: ::c_int = 0;
pub const NFPROTO_IPV4: ::c_int = 2;
pub const NFPROTO_ARP: ::c_int = 3;
pub const NFPROTO_BRIDGE: ::c_int = 7;
pub const NFPROTO_IPV6: ::c_int = 10;
pub const NFPROTO_DECNET: ::c_int = 12;
pub const NFPROTO_NUMPROTO: ::c_int = 13;

// linux/netfilter_ipv4.h
pub const NF_IP_PRE_ROUTING: ::c_int = 0;
pub const NF_IP_LOCAL_IN: ::c_int = 1;
pub const NF_IP_FORWARD: ::c_int = 2;
pub const NF_IP_LOCAL_OUT: ::c_int = 3;
pub const NF_IP_POST_ROUTING: ::c_int = 4;
pub const NF_IP_NUMHOOKS: ::c_int = 5;

pub const NF_IP_PRI_FIRST: ::c_int = ::INT_MIN;
pub const NF_IP_PRI_CONNTRACK_DEFRAG: ::c_int = -400;
pub const NF_IP_PRI_RAW: ::c_int = -300;
pub const NF_IP_PRI_SELINUX_FIRST: ::c_int = -225;
pub const NF_IP_PRI_CONNTRACK: ::c_int = -200;
pub const NF_IP_PRI_MANGLE: ::c_int = -150;
pub const NF_IP_PRI_NAT_DST: ::c_int = -100;
pub const NF_IP_PRI_FILTER: ::c_int = 0;
pub const NF_IP_PRI_SECURITY: ::c_int = 50;
pub const NF_IP_PRI_NAT_SRC: ::c_int = 100;
pub const NF_IP_PRI_SELINUX_LAST: ::c_int = 225;
pub const NF_IP_PRI_CONNTRACK_HELPER: ::c_int = 300;
pub const NF_IP_PRI_CONNTRACK_CONFIRM: ::c_int = ::INT_MAX;
pub const NF_IP_PRI_LAST: ::c_int = ::INT_MAX;

// linux/netfilter_ipv6.h
pub const NF_IP6_PRE_ROUTING: ::c_int = 0;
pub const NF_IP6_LOCAL_IN: ::c_int = 1;
pub const NF_IP6_FORWARD: ::c_int = 2;
pub const NF_IP6_LOCAL_OUT: ::c_int = 3;
pub const NF_IP6_POST_ROUTING: ::c_int = 4;
pub const NF_IP6_NUMHOOKS: ::c_int = 5;

pub const NF_IP6_PRI_FIRST: ::c_int = ::INT_MIN;
pub const NF_IP6_PRI_CONNTRACK_DEFRAG: ::c_int = -400;
pub const NF_IP6_PRI_RAW: ::c_int = -300;
pub const NF_IP6_PRI_SELINUX_FIRST: ::c_int = -225;
pub const NF_IP6_PRI_CONNTRACK: ::c_int = -200;
pub const NF_IP6_PRI_MANGLE: ::c_int = -150;
pub const NF_IP6_PRI_NAT_DST: ::c_int = -100;
pub const NF_IP6_PRI_FILTER: ::c_int = 0;
pub const NF_IP6_PRI_SECURITY: ::c_int = 50;
pub const NF_IP6_PRI_NAT_SRC: ::c_int = 100;
pub const NF_IP6_PRI_SELINUX_LAST: ::c_int = 225;
pub const NF_IP6_PRI_CONNTRACK_HELPER: ::c_int = 300;
pub const NF_IP6_PRI_LAST: ::c_int = ::INT_MAX;

pub const SIOCADDRT: ::c_ulong = 0x0000890B;
pub const SIOCDELRT: ::c_ulong = 0x0000890C;
pub const SIOCGIFNAME: ::c_ulong = 0x00008910;
pub const SIOCSIFLINK: ::c_ulong = 0x00008911;
pub const SIOCGIFCONF: ::c_ulong = 0x00008912;
pub const SIOCGIFFLAGS: ::c_ulong = 0x00008913;
pub const SIOCSIFFLAGS: ::c_ulong = 0x00008914;
pub const SIOCGIFADDR: ::c_ulong = 0x00008915;
pub const SIOCSIFADDR: ::c_ulong = 0x00008916;
pub const SIOCGIFDSTADDR: ::c_ulong = 0x00008917;
pub const SIOCSIFDSTADDR: ::c_ulong = 0x00008918;
pub const SIOCGIFBRDADDR: ::c_ulong = 0x00008919;
pub const SIOCSIFBRDADDR: ::c_ulong = 0x0000891A;
pub const SIOCGIFNETMASK: ::c_ulong = 0x0000891B;
pub const SIOCSIFNETMASK: ::c_ulong = 0x0000891C;
pub const SIOCGIFMETRIC: ::c_ulong = 0x0000891D;
pub const SIOCSIFMETRIC: ::c_ulong = 0x0000891E;
pub const SIOCGIFMEM: ::c_ulong = 0x0000891F;
pub const SIOCSIFMEM: ::c_ulong = 0x00008920;
pub const SIOCGIFMTU: ::c_ulong = 0x00008921;
pub const SIOCSIFMTU: ::c_ulong = 0x00008922;
pub const SIOCSIFHWADDR: ::c_ulong = 0x00008924;
pub const SIOCGIFENCAP: ::c_ulong = 0x00008925;
pub const SIOCSIFENCAP: ::c_ulong = 0x00008926;
pub const SIOCGIFHWADDR: ::c_ulong = 0x00008927;
pub const SIOCGIFSLAVE: ::c_ulong = 0x00008929;
pub const SIOCSIFSLAVE: ::c_ulong = 0x00008930;
pub const SIOCADDMULTI: ::c_ulong = 0x00008931;
pub const SIOCDELMULTI: ::c_ulong = 0x00008932;
pub const SIOCDARP: ::c_ulong = 0x00008953;
pub const SIOCGARP: ::c_ulong = 0x00008954;
pub const SIOCSARP: ::c_ulong = 0x00008955;
pub const SIOCDRARP: ::c_ulong = 0x00008960;
pub const SIOCGRARP: ::c_ulong = 0x00008961;
pub const SIOCSRARP: ::c_ulong = 0x00008962;
pub const SIOCGIFMAP: ::c_ulong = 0x00008970;
pub const SIOCSIFMAP: ::c_ulong = 0x00008971;

pub const IPTOS_TOS_MASK: u8 = 0x1E;
pub const IPTOS_PREC_MASK: u8 = 0xE0;

pub const RTF_UP: ::c_ushort = 0x0001;
pub const RTF_GATEWAY: ::c_ushort = 0x0002;

pub const RTF_HOST: ::c_ushort = 0x0004;
pub const RTF_REINSTATE: ::c_ushort = 0x0008;
pub const RTF_DYNAMIC: ::c_ushort = 0x0010;
pub const RTF_MODIFIED: ::c_ushort = 0x0020;
pub const RTF_MTU: ::c_ushort = 0x0040;
pub const RTF_MSS: ::c_ushort = RTF_MTU;
pub const RTF_WINDOW: ::c_ushort = 0x0080;
pub const RTF_IRTT: ::c_ushort = 0x0100;
pub const RTF_REJECT: ::c_ushort = 0x0200;
pub const RTF_STATIC: ::c_ushort = 0x0400;
pub const RTF_XRESOLVE: ::c_ushort = 0x0800;
pub const RTF_NOFORWARD: ::c_ushort = 0x1000;
pub const RTF_THROW: ::c_ushort = 0x2000;
pub const RTF_NOPMTUDISC: ::c_ushort = 0x4000;

pub const RTF_DEFAULT: u32 = 0x00010000;
pub const RTF_ALLONLINK: u32 = 0x00020000;
pub const RTF_ADDRCONF: u32 = 0x00040000;
pub const RTF_LINKRT: u32 = 0x00100000;
pub const RTF_NONEXTHOP: u32 = 0x00200000;
pub const RTF_CACHE: u32 = 0x01000000;
pub const RTF_FLOW: u32 = 0x02000000;
pub const RTF_POLICY: u32 = 0x04000000;

pub const RTCF_VALVE: u32 = 0x00200000;
pub const RTCF_MASQ: u32 = 0x00400000;
pub const RTCF_NAT: u32 = 0x00800000;
pub const RTCF_DOREDIRECT: u32 = 0x01000000;
pub const RTCF_LOG: u32 = 0x02000000;
pub const RTCF_DIRECTSRC: u32 = 0x04000000;

pub const RTF_LOCAL: u32 = 0x80000000;
pub const RTF_INTERFACE: u32 = 0x40000000;
pub const RTF_MULTICAST: u32 = 0x20000000;
pub const RTF_BROADCAST: u32 = 0x10000000;
pub const RTF_NAT: u32 = 0x08000000;
pub const RTF_ADDRCLASSMASK: u32 = 0xF8000000;

pub const RT_CLASS_UNSPEC: u8 = 0;
pub const RT_CLASS_DEFAULT: u8 = 253;
pub const RT_CLASS_MAIN: u8 = 254;
pub const RT_CLASS_LOCAL: u8 = 255;
pub const RT_CLASS_MAX: u8 = 255;

pub const RTMSG_OVERRUN: u32 = ::NLMSG_OVERRUN as u32;
pub const RTMSG_NEWDEVICE: u32 = 0x11;
pub const RTMSG_DELDEVICE: u32 = 0x12;
pub const RTMSG_NEWROUTE: u32 = 0x21;
pub const RTMSG_DELROUTE: u32 = 0x22;
pub const RTMSG_NEWRULE: u32 = 0x31;
pub const RTMSG_DELRULE: u32 = 0x32;
pub const RTMSG_CONTROL: u32 = 0x40;
pub const RTMSG_AR_FAILED: u32 = 0x51;

pub const MAX_ADDR_LEN: usize = 7;
pub const ARPD_UPDATE: ::c_ushort = 0x01;
pub const ARPD_LOOKUP: ::c_ushort = 0x02;
pub const ARPD_FLUSH: ::c_ushort = 0x03;
pub const ATF_MAGIC: ::c_int = 0x80;

f! {
    pub fn CPU_ZERO(cpuset: &mut ::cpu_set_t) -> () {
        for slot in cpuset.bits.iter_mut() {
            *slot = 0;
        }
    }

    pub fn CPU_SET(cpu: usize, cpuset: &mut ::cpu_set_t) -> () {
        let size_in_bits = 8 * mem::size_of_val(&cpuset.bits[0]); // 32, 64 etc
        let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
        cpuset.bits[idx] |= 1 << offset;
        ()
    }

    pub fn CPU_CLR(cpu: usize, cpuset: &mut ::cpu_set_t) -> () {
        let size_in_bits = 8 * mem::size_of_val(&cpuset.bits[0]); // 32, 64 etc
        let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
        cpuset.bits[idx] &= !(1 << offset);
        ()
    }

    pub fn CPU_ISSET(cpu: usize, cpuset: &::cpu_set_t) -> bool {
        let size_in_bits = 8 * mem::size_of_val(&cpuset.bits[0]);
        let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
        0 != (cpuset.bits[idx] & (1 << offset))
    }

    pub fn CPU_EQUAL(set1: &::cpu_set_t, set2: &::cpu_set_t) -> bool {
        set1.bits == set2.bits
    }

    pub fn major(dev: ::dev_t) -> ::c_uint {
        let mut major = 0;
        major |= (dev & 0x00000000000fff00) >> 8;
        major |= (dev & 0xfffff00000000000) >> 32;
        major as ::c_uint
    }

    pub fn minor(dev: ::dev_t) -> ::c_uint {
        let mut minor = 0;
        minor |= (dev & 0x00000000000000ff) >> 0;
        minor |= (dev & 0x00000ffffff00000) >> 12;
        minor as ::c_uint
    }

    pub fn makedev(major: ::c_uint, minor: ::c_uint) -> ::dev_t {
        let major = major as ::dev_t;
        let minor = minor as ::dev_t;
        let mut dev = 0;
        dev |= (major & 0x00000fff) << 8;
        dev |= (major & 0xfffff000) << 32;
        dev |= (minor & 0x000000ff) << 0;
        dev |= (minor & 0xffffff00) << 12;
        dev
    }

    pub fn IPTOS_TOS(tos: u8) -> u8 {
        tos & IPTOS_TOS_MASK
    }

    pub fn IPTOS_PREC(tos: u8) -> u8 {
        tos & IPTOS_PREC_MASK
    }

    pub fn RT_TOS(tos: u8) -> u8 {
        tos & ::IPTOS_TOS_MASK
    }

    pub fn RT_ADDRCLASS(flags: u32) -> u32 {
        flags >> 23
    }

    pub fn RT_LOCALADDR(flags: u32) -> bool {
        (flags & RTF_ADDRCLASSMASK) == (RTF_LOCAL | RTF_INTERFACE)
    }

    pub fn NLA_ALIGN(len: ::c_int) -> ::c_int {
        return ((len) + NLA_ALIGNTO - 1) & !(NLA_ALIGNTO - 1)
    }
}

extern {
    pub fn ioctl(fd: ::c_int, request: ::c_ulong, ...) -> ::c_int;
    pub fn backtrace(buf: *mut *mut ::c_void,
                     sz: ::c_int) -> ::c_int;
    pub fn glob64(pattern: *const ::c_char,
                  flags: ::c_int,
                  errfunc: ::dox::Option<extern fn(epath: *const ::c_char,
                                                   errno: ::c_int)
                                                   -> ::c_int>,
                  pglob: *mut glob64_t) -> ::c_int;
    pub fn globfree64(pglob: *mut glob64_t);
    pub fn ptrace(request: ::c_int, ...) -> ::c_long;
    pub fn pthread_attr_getaffinity_np(attr: *const ::pthread_attr_t,
                                       cpusetsize: ::size_t,
                                       cpuset: *mut ::cpu_set_t) -> ::c_int;
    pub fn pthread_attr_setaffinity_np(attr: *mut ::pthread_attr_t,
                                       cpusetsize: ::size_t,
                                       cpuset: *const ::cpu_set_t) -> ::c_int;
    pub fn getpriority(which: ::__priority_which_t, who: ::id_t) -> ::c_int;
    pub fn setpriority(which: ::__priority_which_t, who: ::id_t,
                                       prio: ::c_int) -> ::c_int;
    pub fn pthread_getaffinity_np(thread: ::pthread_t,
                                  cpusetsize: ::size_t,
                                  cpuset: *mut ::cpu_set_t) -> ::c_int;
    pub fn pthread_setaffinity_np(thread: ::pthread_t,
                                  cpusetsize: ::size_t,
                                  cpuset: *const ::cpu_set_t) -> ::c_int;
    pub fn sched_getcpu() -> ::c_int;
    pub fn mallinfo() -> ::mallinfo;
    pub fn getpwent_r(pwd: *mut ::passwd,
                      buf: *mut ::c_char,
                      buflen: ::size_t,
                      result: *mut *mut ::passwd) -> ::c_int;
    pub fn getgrent_r(grp: *mut ::group,
                      buf: *mut ::c_char,
                      buflen: ::size_t,
                      result: *mut *mut ::group) -> ::c_int;
    pub fn setgrent();
    pub fn endgrent();
    pub fn getgrent() -> *mut ::group;

    // System V IPC
    pub fn semget(key: ::key_t, nsems: ::c_int, semflag: ::c_int) -> ::c_int;
    pub fn semop(semid: ::c_int,
                 sops: *mut ::sembuf, nsops: ::size_t) -> ::c_int;
    pub fn semctl(semid: ::c_int,
                  semnum: ::c_int, cmd: ::c_int, ...) -> ::c_int;

    pub fn fallocate(fd: ::c_int, mode: ::c_int,
                     offset: ::off_t, len: ::off_t) -> ::c_int;
    pub fn posix_fallocate(fd: ::c_int, offset: ::off_t,
                           len: ::off_t) -> ::c_int;
    pub fn getxattr(path: *const c_char, name: *const c_char,
                    value: *mut ::c_void, size: ::size_t) -> ::ssize_t;
    pub fn lgetxattr(path: *const c_char, name: *const c_char,
                     value: *mut ::c_void, size: ::size_t) -> ::ssize_t;
    pub fn fgetxattr(filedes: ::c_int, name: *const c_char,
                     value: *mut ::c_void, size: ::size_t) -> ::ssize_t;
    pub fn setxattr(path: *const c_char, name: *const c_char,
                    value: *const ::c_void, size: ::size_t,
                    flags: ::c_int) -> ::c_int;
    pub fn lsetxattr(path: *const c_char, name: *const c_char,
                     value: *const ::c_void, size: ::size_t,
                     flags: ::c_int) -> ::c_int;
    pub fn fsetxattr(filedes: ::c_int, name: *const c_char,
                     value: *const ::c_void, size: ::size_t,
                     flags: ::c_int) -> ::c_int;
    pub fn listxattr(path: *const c_char, list: *mut c_char,
                     size: ::size_t) -> ::ssize_t;
    pub fn llistxattr(path: *const c_char, list: *mut c_char,
                      size: ::size_t) -> ::ssize_t;
    pub fn flistxattr(filedes: ::c_int, list: *mut c_char,
                      size: ::size_t) -> ::ssize_t;
    pub fn removexattr(path: *const c_char, name: *const c_char) -> ::c_int;
    pub fn lremovexattr(path: *const c_char, name: *const c_char) -> ::c_int;
    pub fn fremovexattr(filedes: ::c_int, name: *const c_char) -> ::c_int;
    pub fn timerfd_create(clockid: ::c_int, flags: ::c_int) -> ::c_int;
    pub fn timerfd_gettime(fd: ::c_int,
                           curr_value: *mut itimerspec) -> ::c_int;
    pub fn timerfd_settime(fd: ::c_int,
                           flags: ::c_int,
                           new_value: *const itimerspec,
                           old_value: *mut itimerspec) -> ::c_int;
    pub fn dup3(oldfd: ::c_int, newfd: ::c_int, flags: ::c_int) -> ::c_int;
    pub fn mkostemp(template: *mut ::c_char, flags: ::c_int) -> ::c_int;
    pub fn mkostemps(template: *mut ::c_char,
                     suffixlen: ::c_int,
                     flags: ::c_int) -> ::c_int;
    pub fn nl_langinfo_l(item: ::nl_item, locale: ::locale_t) -> *mut ::c_char;
    pub fn pthread_setschedprio(native: ::pthread_t,
                                priority: ::c_int) -> ::c_int;
    pub fn posix_madvise(addr: *mut ::c_void, len: ::size_t, advice: ::c_int)
                         -> ::c_int;

    pub fn remap_file_pages(addr: *mut ::c_void, size: ::size_t, prot: ::c_int,
                            pgoff: ::size_t, flags: ::c_int) -> ::c_int;
    pub fn mkstemps(template: *mut ::c_char, suffixlen: ::c_int) -> ::c_int;

    pub fn getdomainname(name: *mut ::c_char, len: ::size_t) -> ::c_int;
    pub fn setdomainname(name: *const ::c_char, len: ::size_t) -> ::c_int;
    pub fn vhangup() -> ::c_int;
    pub fn sync();
    pub fn pthread_getschedparam(native: ::pthread_t,
                                 policy: *mut ::c_int,
                                 param: *mut ::sched_param) -> ::c_int;
    pub fn settimeofday(tv: *const ::timeval, tz: *const ::timezone) -> ::c_int;
    pub fn eventfd(init: ::c_uint, flags: ::c_int) -> ::c_int;
    pub fn sched_rr_get_interval(pid: ::pid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn sched_setparam(pid: ::pid_t, param: *const ::sched_param) -> ::c_int;
    pub fn setns(fd: ::c_int, nstype: ::c_int) -> ::c_int;
    pub fn swapoff(puath: *const ::c_char) -> ::c_int;
    pub fn personality(persona: ::c_ulong) -> ::c_int;
    pub fn sched_getparam(pid: ::pid_t, param: *mut ::sched_param) -> ::c_int;
    pub fn sysinfo(info: *mut ::sysinfo) -> ::c_int;
    pub fn pthread_setschedparam(native: ::pthread_t,
                                 policy: ::c_int,
                                 param: *const ::sched_param) -> ::c_int;
    pub fn swapon(path: *const ::c_char, swapflags: ::c_int) -> ::c_int;
    pub fn sigsuspend(mask: *const ::sigset_t) -> ::c_int;
    pub fn pthread_cancel(thread: ::pthread_t) -> ::c_int;
    pub fn getgrouplist(user: *const ::c_char,
                        group: ::gid_t,
                        groups: *mut ::gid_t,
                        ngroups: *mut ::c_int) -> ::c_int;
    pub fn faccessat(dirfd: ::c_int, pathname: *const ::c_char,
                     mode: ::c_int, flags: ::c_int) -> ::c_int;

    pub fn setmntent(filename: *const ::c_char,
                     ty: *const ::c_char) -> *mut ::FILE;
    pub fn getmntent(stream: *mut ::FILE) -> *mut ::mntent;
    pub fn addmntent(stream: *mut ::FILE, mnt: *const ::mntent) -> ::c_int;
    pub fn endmntent(streamp: *mut ::FILE) -> ::c_int;
    pub fn hasmntopt(mnt: *const ::mntent,
                     opt: *const ::c_char) -> *mut ::c_char;

    pub fn fread_unlocked(ptr: *mut ::c_void,
        size: ::size_t,
        nobj: ::size_t,
        stream: *mut ::FILE
    ) -> ::size_t;
    pub fn sysctl(name: *mut ::c_int,
                  namelen: ::c_int,
                  oldp: *mut ::c_void,
                  oldlenp: *mut ::size_t,
                  newp: *mut ::c_void,
                  newlen: ::size_t)
                  -> ::c_int;
}

pub type __priority_which_t = ::c_uint;

s! {
    pub struct sigaction {
        pub sa_sigaction: ::sighandler_t,
        pub sa_flags: ::c_long,
        pub sa_restorer: ::dox::Option<extern fn()>,
        pub sa_mask: ::sigset_t,
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_flags: ::c_int,
        pub ss_size: ::size_t
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_errno: ::c_int,
        pub si_code: ::c_int,
        pub _pad: [::c_int; 29],
        #[cfg(target_arch = "x86_64")]
        _align: [u64; 0],
        #[cfg(not(target_arch = "x86_64"))]
        _align: [usize; 0],
    }

    pub struct glob64_t {
        pub gl_pathc: ::size_t,
        pub gl_pathv: *mut *mut ::c_char,
        pub gl_offs: ::size_t,
        pub gl_flags: ::c_int,

        __unused1: *mut ::c_void,
        __unused2: *mut ::c_void,
        __unused3: *mut ::c_void,
        __unused4: *mut ::c_void,
        __unused5: *mut ::c_void,
    }

    pub struct statfs {
        pub f_type: ::c_int,
        pub f_bsize: ::c_int,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,

        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_fsid: ::fsid_t,

        pub f_namelen: ::c_int,
        pub f_frsize: ::c_int,
        f_spare: [::c_int; 5],
    }

    pub struct msghdr {
        pub msg_name: *mut ::c_void,
        pub msg_namelen: ::socklen_t,
        pub msg_iov: *mut ::iovec,
        pub msg_iovlen: ::c_int,
        pub msg_control: *mut ::c_void,
        pub msg_controllen: ::size_t,
        pub msg_flags: ::c_int,
    }

    pub struct cmsghdr {
        pub cmsg_len: ::size_t,
        pub cmsg_level: ::c_int,
        pub cmsg_type: ::c_int,
    }

    pub struct termios {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_line: ::cc_t,
        pub c_cc: [::cc_t; ::NCCS],
        #[cfg(not(target_arch = "sparc64"))]
        pub c_ispeed: ::speed_t,
        #[cfg(not(target_arch = "sparc64"))]
        pub c_ospeed: ::speed_t,
    }

    pub struct flock {
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
        pub l_start: ::off_t,
        pub l_len: ::off_t,
        pub l_pid: ::pid_t,
    }

    // FIXME this is actually a union
    pub struct sem_t {
        #[cfg(target_pointer_width = "32")]
        __size: [::c_char; 16],
        #[cfg(target_pointer_width = "64")]
        __size: [::c_char; 32],
        __align: [::c_long; 0],
    }

    pub struct mallinfo {
        pub arena: ::c_int,
        pub ordblks: ::c_int,
        pub smblks: ::c_int,
        pub hblks: ::c_int,
        pub hblkhd: ::c_int,
        pub usmblks: ::c_int,
        pub fsmblks: ::c_int,
        pub uordblks: ::c_int,
        pub fordblks: ::c_int,
        pub keepcost: ::c_int,
    }

    pub struct nlmsghdr {
        pub nlmsg_len: u32,
        pub nlmsg_type: u16,
        pub nlmsg_flags: u16,
        pub nlmsg_seq: u32,
        pub nlmsg_pid: u32,
    }

    pub struct nlmsgerr {
        pub error: ::c_int,
        pub msg: nlmsghdr,
    }

    pub struct nl_pktinfo {
        pub group: u32,
    }

    pub struct nl_mmap_req {
        pub nm_block_size: ::c_uint,
        pub nm_block_nr: ::c_uint,
        pub nm_frame_size: ::c_uint,
        pub nm_frame_nr: ::c_uint,
    }

    pub struct nl_mmap_hdr {
        pub nm_status: ::c_uint,
        pub nm_len: ::c_uint,
        pub nm_group: u32,
        pub nm_pid: u32,
        pub nm_uid: u32,
        pub nm_gid: u32,
    }

    pub struct nlattr {
        pub nla_len: u16,
        pub nla_type: u16,
    }

    pub struct rtentry {
        pub rt_pad1: ::c_ulong,
        pub rt_dst: ::sockaddr,
        pub rt_gateway: ::sockaddr,
        pub rt_genmask: ::sockaddr,
        pub rt_flags: ::c_ushort,
        pub rt_pad2: ::c_short,
        pub rt_pad3: ::c_ulong,
        pub rt_tos: ::c_uchar,
        pub rt_class: ::c_uchar,
        #[cfg(target_pointer_width = "64")]
        pub rt_pad4: [::c_short; 3usize],
        #[cfg(not(target_pointer_width = "64"))]
        pub rt_pad4: ::c_short,
        pub rt_metric: ::c_short,
        pub rt_dev: *mut ::c_char,
        pub rt_mtu: ::c_ulong,
        pub rt_window: ::c_ulong,
        pub rt_irtt: ::c_ushort,
    }
}

pub const RLIMIT_RSS: ::c_int = 5;
pub const RLIMIT_AS: ::c_int = 9;
pub const RLIMIT_MEMLOCK: ::c_int = 8;
pub const RLIM_INFINITY: ::rlim_t = !0;
pub const RLIMIT_NLIMITS: ::c_int = 15;

pub const SOCK_NONBLOCK: ::c_int = O_NONBLOCK;

pub const MSG_TRYHARD: ::c_int = 4;

pub const MAP_ANON: ::c_int = 0x0020;
pub const MAP_ANONYMOUS: ::c_int = 0x0020;
pub const MAP_DENYWRITE: ::c_int = 0x0800;
pub const MAP_EXECUTABLE: ::c_int = 0x01000;
pub const MAP_POPULATE: ::c_int = 0x08000;
pub const MAP_NONBLOCK: ::c_int = 0x010000;
pub const MAP_STACK: ::c_int = 0x020000;

pub const ENOTSUP: ::c_int = EOPNOTSUPP;
pub const EUCLEAN: ::c_int = 117;
pub const ENOTNAM: ::c_int = 118;
pub const ENAVAIL: ::c_int = 119;
pub const EISNAM: ::c_int = 120;
pub const EREMOTEIO: ::c_int = 121;

pub const SOCK_STREAM: ::c_int = 1;
pub const SOCK_DGRAM: ::c_int = 2;
pub const SOCK_SEQPACKET: ::c_int = 5;
pub const SOCK_DCCP: ::c_int = 6;
pub const SOCK_PACKET: ::c_int = 10;

pub const TCP_COOKIE_TRANSACTIONS: ::c_int = 15;
pub const TCP_THIN_LINEAR_TIMEOUTS: ::c_int = 16;
pub const TCP_THIN_DUPACK: ::c_int = 17;
pub const TCP_USER_TIMEOUT: ::c_int = 18;
pub const TCP_REPAIR: ::c_int = 19;
pub const TCP_REPAIR_QUEUE: ::c_int = 20;
pub const TCP_QUEUE_SEQ: ::c_int = 21;
pub const TCP_REPAIR_OPTIONS: ::c_int = 22;
pub const TCP_FASTOPEN: ::c_int = 23;
pub const TCP_TIMESTAMP: ::c_int = 24;

pub const SIGTTIN: ::c_int = 21;
pub const SIGTTOU: ::c_int = 22;
pub const SIGXCPU: ::c_int = 24;
pub const SIGXFSZ: ::c_int = 25;
pub const SIGVTALRM: ::c_int = 26;
pub const SIGPROF: ::c_int = 27;
pub const SIGWINCH: ::c_int = 28;

pub const SIGEV_THREAD_ID: ::c_int = 4;

pub const BUFSIZ: ::c_uint = 4096;
pub const TMP_MAX: ::c_uint = 238328;
pub const FOPEN_MAX: ::c_uint = 16;
pub const POSIX_FADV_DONTNEED: ::c_int = 4;
pub const POSIX_FADV_NOREUSE: ::c_int = 5;
pub const POSIX_MADV_DONTNEED: ::c_int = 4;
pub const _SC_EQUIV_CLASS_MAX: ::c_int = 41;
pub const _SC_CHARCLASS_NAME_MAX: ::c_int = 45;
pub const _SC_PII: ::c_int = 53;
pub const _SC_PII_XTI: ::c_int = 54;
pub const _SC_PII_SOCKET: ::c_int = 55;
pub const _SC_PII_INTERNET: ::c_int = 56;
pub const _SC_PII_OSI: ::c_int = 57;
pub const _SC_POLL: ::c_int = 58;
pub const _SC_SELECT: ::c_int = 59;
pub const _SC_PII_INTERNET_STREAM: ::c_int = 61;
pub const _SC_PII_INTERNET_DGRAM: ::c_int = 62;
pub const _SC_PII_OSI_COTS: ::c_int = 63;
pub const _SC_PII_OSI_CLTS: ::c_int = 64;
pub const _SC_PII_OSI_M: ::c_int = 65;
pub const _SC_T_IOV_MAX: ::c_int = 66;
pub const _SC_2_C_VERSION: ::c_int = 96;
pub const _SC_CHAR_BIT: ::c_int = 101;
pub const _SC_CHAR_MAX: ::c_int = 102;
pub const _SC_CHAR_MIN: ::c_int = 103;
pub const _SC_INT_MAX: ::c_int = 104;
pub const _SC_INT_MIN: ::c_int = 105;
pub const _SC_LONG_BIT: ::c_int = 106;
pub const _SC_WORD_BIT: ::c_int = 107;
pub const _SC_MB_LEN_MAX: ::c_int = 108;
pub const _SC_SSIZE_MAX: ::c_int = 110;
pub const _SC_SCHAR_MAX: ::c_int = 111;
pub const _SC_SCHAR_MIN: ::c_int = 112;
pub const _SC_SHRT_MAX: ::c_int = 113;
pub const _SC_SHRT_MIN: ::c_int = 114;
pub const _SC_UCHAR_MAX: ::c_int = 115;
pub const _SC_UINT_MAX: ::c_int = 116;
pub const _SC_ULONG_MAX: ::c_int = 117;
pub const _SC_USHRT_MAX: ::c_int = 118;
pub const _SC_NL_ARGMAX: ::c_int = 119;
pub const _SC_NL_LANGMAX: ::c_int = 120;
pub const _SC_NL_MSGMAX: ::c_int = 121;
pub const _SC_NL_NMAX: ::c_int = 122;
pub const _SC_NL_SETMAX: ::c_int = 123;
pub const _SC_NL_TEXTMAX: ::c_int = 124;
pub const _SC_BASE: ::c_int = 134;
pub const _SC_C_LANG_SUPPORT: ::c_int = 135;
pub const _SC_C_LANG_SUPPORT_R: ::c_int = 136;
pub const _SC_DEVICE_IO: ::c_int = 140;
pub const _SC_DEVICE_SPECIFIC: ::c_int = 141;
pub const _SC_DEVICE_SPECIFIC_R: ::c_int = 142;
pub const _SC_FD_MGMT: ::c_int = 143;
pub const _SC_FIFO: ::c_int = 144;
pub const _SC_PIPE: ::c_int = 145;
pub const _SC_FILE_ATTRIBUTES: ::c_int = 146;
pub const _SC_FILE_LOCKING: ::c_int = 147;
pub const _SC_FILE_SYSTEM: ::c_int = 148;
pub const _SC_MULTI_PROCESS: ::c_int = 150;
pub const _SC_SINGLE_PROCESS: ::c_int = 151;
pub const _SC_NETWORKING: ::c_int = 152;
pub const _SC_REGEX_VERSION: ::c_int = 156;
pub const _SC_SIGNALS: ::c_int = 158;
pub const _SC_SYSTEM_DATABASE: ::c_int = 162;
pub const _SC_SYSTEM_DATABASE_R: ::c_int = 163;
pub const _SC_USER_GROUPS: ::c_int = 166;
pub const _SC_USER_GROUPS_R: ::c_int = 167;
pub const _SC_LEVEL1_ICACHE_SIZE: ::c_int = 185;
pub const _SC_LEVEL1_ICACHE_ASSOC: ::c_int = 186;
pub const _SC_LEVEL1_ICACHE_LINESIZE: ::c_int = 187;
pub const _SC_LEVEL1_DCACHE_SIZE: ::c_int = 188;
pub const _SC_LEVEL1_DCACHE_ASSOC: ::c_int = 189;
pub const _SC_LEVEL1_DCACHE_LINESIZE: ::c_int = 190;
pub const _SC_LEVEL2_CACHE_SIZE: ::c_int = 191;
pub const _SC_LEVEL2_CACHE_ASSOC: ::c_int = 192;
pub const _SC_LEVEL2_CACHE_LINESIZE: ::c_int = 193;
pub const _SC_LEVEL3_CACHE_SIZE: ::c_int = 194;
pub const _SC_LEVEL3_CACHE_ASSOC: ::c_int = 195;
pub const _SC_LEVEL3_CACHE_LINESIZE: ::c_int = 196;
pub const _SC_LEVEL4_CACHE_SIZE: ::c_int = 197;
pub const _SC_LEVEL4_CACHE_ASSOC: ::c_int = 198;
pub const _SC_LEVEL4_CACHE_LINESIZE: ::c_int = 199;
pub const O_ACCMODE: ::c_int = 3;
pub const ST_RELATIME: ::c_ulong = 4096;
pub const NI_MAXHOST: ::socklen_t = 1025;

pub const ADFS_SUPER_MAGIC: ::c_long = 0x0000adf5;
pub const AFFS_SUPER_MAGIC: ::c_long = 0x0000adff;
pub const CODA_SUPER_MAGIC: ::c_long = 0x73757245;
pub const CRAMFS_MAGIC: ::c_long = 0x28cd3d45;
pub const EFS_SUPER_MAGIC: ::c_long = 0x00414a53;
pub const EXT2_SUPER_MAGIC: ::c_long = 0x0000ef53;
pub const EXT3_SUPER_MAGIC: ::c_long = 0x0000ef53;
pub const EXT4_SUPER_MAGIC: ::c_long = 0x0000ef53;
pub const HPFS_SUPER_MAGIC: ::c_long = 0xf995e849;
pub const HUGETLBFS_MAGIC: ::c_long = 0x958458f6;
pub const ISOFS_SUPER_MAGIC: ::c_long = 0x00009660;
pub const JFFS2_SUPER_MAGIC: ::c_long = 0x000072b6;
pub const MINIX_SUPER_MAGIC: ::c_long = 0x0000137f;
pub const MINIX_SUPER_MAGIC2: ::c_long = 0x0000138f;
pub const MINIX2_SUPER_MAGIC: ::c_long = 0x00002468;
pub const MINIX2_SUPER_MAGIC2: ::c_long = 0x00002478;
pub const MSDOS_SUPER_MAGIC: ::c_long = 0x00004d44;
pub const NCP_SUPER_MAGIC: ::c_long = 0x0000564c;
pub const NFS_SUPER_MAGIC: ::c_long = 0x00006969;
pub const OPENPROM_SUPER_MAGIC: ::c_long = 0x00009fa1;
pub const PROC_SUPER_MAGIC: ::c_long = 0x00009fa0;
pub const QNX4_SUPER_MAGIC: ::c_long = 0x0000002f;
pub const REISERFS_SUPER_MAGIC: ::c_long = 0x52654973;
pub const SMB_SUPER_MAGIC: ::c_long = 0x0000517b;
pub const TMPFS_MAGIC: ::c_long = 0x01021994;
pub const USBDEVICE_SUPER_MAGIC: ::c_long = 0x00009fa2;

pub const VEOF: usize = 4;

pub const CPU_SETSIZE: ::c_int = 0x400;

pub const PTRACE_TRACEME: ::c_uint = 0;
pub const PTRACE_PEEKTEXT: ::c_uint = 1;
pub const PTRACE_PEEKDATA: ::c_uint = 2;
pub const PTRACE_PEEKUSER: ::c_uint = 3;
pub const PTRACE_POKETEXT: ::c_uint = 4;
pub const PTRACE_POKEDATA: ::c_uint = 5;
pub const PTRACE_POKEUSER: ::c_uint = 6;
pub const PTRACE_CONT: ::c_uint = 7;
pub const PTRACE_KILL: ::c_uint = 8;
pub const PTRACE_SINGLESTEP: ::c_uint = 9;
pub const PTRACE_ATTACH: ::c_uint = 16;
pub const PTRACE_SYSCALL: ::c_uint = 24;
pub const PTRACE_SETOPTIONS: ::c_uint = 0x4200;
pub const PTRACE_GETEVENTMSG: ::c_uint = 0x4201;
pub const PTRACE_GETSIGINFO: ::c_uint = 0x4202;
pub const PTRACE_SETSIGINFO: ::c_uint = 0x4203;
pub const PTRACE_GETREGSET: ::c_uint = 0x4204;
pub const PTRACE_SETREGSET: ::c_uint = 0x4205;
pub const PTRACE_SEIZE: ::c_uint = 0x4206;
pub const PTRACE_INTERRUPT: ::c_uint = 0x4207;
pub const PTRACE_LISTEN: ::c_uint = 0x4208;
pub const PTRACE_PEEKSIGINFO: ::c_uint = 0x4209;

pub const MAP_HUGETLB: ::c_int = 0x040000;

pub const SEEK_DATA: ::c_int = 3;
pub const SEEK_HOLE: ::c_int = 4;

pub const TCSANOW: ::c_int = 0;
pub const TCSADRAIN: ::c_int = 1;
pub const TCSAFLUSH: ::c_int = 2;

pub const TIOCLINUX: ::c_ulong = 0x541C;
pub const TIOCGSERIAL: ::c_ulong = 0x541E;

pub const RTLD_DEEPBIND: ::c_int = 0x8;
pub const RTLD_GLOBAL: ::c_int = 0x100;
pub const RTLD_NOLOAD: ::c_int = 0x4;

pub const LINUX_REBOOT_MAGIC1: ::c_int = 0xfee1dead;
pub const LINUX_REBOOT_MAGIC2: ::c_int = 672274793;
pub const LINUX_REBOOT_MAGIC2A: ::c_int = 85072278;
pub const LINUX_REBOOT_MAGIC2B: ::c_int = 369367448;
pub const LINUX_REBOOT_MAGIC2C: ::c_int = 537993216;

pub const LINUX_REBOOT_CMD_RESTART: ::c_int = 0x01234567;
pub const LINUX_REBOOT_CMD_HALT: ::c_int = 0xCDEF0123;
pub const LINUX_REBOOT_CMD_CAD_ON: ::c_int = 0x89ABCDEF;
pub const LINUX_REBOOT_CMD_CAD_OFF: ::c_int = 0x00000000;
pub const LINUX_REBOOT_CMD_POWER_OFF: ::c_int = 0x4321FEDC;
pub const LINUX_REBOOT_CMD_RESTART2: ::c_int = 0xA1B2C3D4;
pub const LINUX_REBOOT_CMD_SW_SUSPEND: ::c_int = 0xD000FCE2;
pub const LINUX_REBOOT_CMD_KEXEC: ::c_int = 0x45584543;

pub const NETLINK_ROUTE: ::c_int = 0;
pub const NETLINK_UNUSED: ::c_int = 1;
pub const NETLINK_USERSOCK: ::c_int = 2;
pub const NETLINK_FIREWALL: ::c_int = 3;
pub const NETLINK_SOCK_DIAG: ::c_int = 4;
pub const NETLINK_NFLOG: ::c_int = 5;
pub const NETLINK_XFRM: ::c_int = 6;
pub const NETLINK_SELINUX: ::c_int = 7;
pub const NETLINK_ISCSI: ::c_int = 8;
pub const NETLINK_AUDIT: ::c_int = 9;
pub const NETLINK_FIB_LOOKUP: ::c_int = 10;
pub const NETLINK_CONNECTOR: ::c_int = 11;
pub const NETLINK_NETFILTER: ::c_int = 12;
pub const NETLINK_IP6_FW: ::c_int = 13;
pub const NETLINK_DNRTMSG: ::c_int = 14;
pub const NETLINK_KOBJECT_UEVENT: ::c_int = 15;
pub const NETLINK_GENERIC: ::c_int = 16;
pub const NETLINK_SCSITRANSPORT: ::c_int = 18;
pub const NETLINK_ECRYPTFS: ::c_int = 19;
pub const NETLINK_RDMA: ::c_int = 20;
pub const NETLINK_CRYPTO: ::c_int = 21;
pub const NETLINK_INET_DIAG: ::c_int = NETLINK_SOCK_DIAG;

pub const MAX_LINKS: ::c_int = 32;

pub const NLM_F_REQUEST: ::c_int = 1;
pub const NLM_F_MULTI: ::c_int = 2;
pub const NLM_F_ACK: ::c_int = 4;
pub const NLM_F_ECHO: ::c_int = 8;
pub const NLM_F_DUMP_INTR: ::c_int = 16;

pub const NLM_F_ROOT: ::c_int = 0x100;
pub const NLM_F_MATCH: ::c_int = 0x200;
pub const NLM_F_ATOMIC: ::c_int = 0x400;
pub const NLM_F_DUMP: ::c_int = NLM_F_ROOT | NLM_F_MATCH;

pub const NLM_F_REPLACE: ::c_int = 0x100;
pub const NLM_F_EXCL: ::c_int = 0x200;
pub const NLM_F_CREATE: ::c_int = 0x400;
pub const NLM_F_APPEND: ::c_int = 0x800;

pub const NETLINK_ADD_MEMBERSHIP: ::c_int = 1;
pub const NETLINK_DROP_MEMBERSHIP: ::c_int = 2;
pub const NETLINK_PKTINFO: ::c_int = 3;
pub const NETLINK_BROADCAST_ERROR: ::c_int = 4;
pub const NETLINK_NO_ENOBUFS: ::c_int = 5;
pub const NETLINK_RX_RING: ::c_int = 6;
pub const NETLINK_TX_RING: ::c_int = 7;

pub const NLA_F_NESTED: ::c_int = 1 << 15;
pub const NLA_F_NET_BYTEORDER: ::c_int = 1 << 14;
pub const NLA_TYPE_MASK: ::c_int = !(NLA_F_NESTED | NLA_F_NET_BYTEORDER);

pub const NLA_ALIGNTO: ::c_int = 4;

pub const GENL_ID_VFS_DQUOT: ::c_int = ::NLMSG_MIN_TYPE + 1;
pub const GENL_ID_PMCRAID: ::c_int = ::NLMSG_MIN_TYPE + 2;

pub const TIOCM_LE: ::c_int = 0x001;
pub const TIOCM_DTR: ::c_int = 0x002;
pub const TIOCM_RTS: ::c_int = 0x004;
pub const TIOCM_ST: ::c_int = 0x008;
pub const TIOCM_SR: ::c_int = 0x010;
pub const TIOCM_CTS: ::c_int = 0x020;
pub const TIOCM_CAR: ::c_int = 0x040;
pub const TIOCM_RNG: ::c_int = 0x080;
pub const TIOCM_DSR: ::c_int = 0x100;
pub const TIOCM_CD: ::c_int = TIOCM_CAR;
pub const TIOCM_RI: ::c_int = TIOCM_RNG;

pub const NFPROTO_INET: ::c_int = 1;

#[doc(hidden)]
pub const AF_MAX: ::c_int = 42;
#[doc(hidden)]
pub const PF_MAX: ::c_int = AF_MAX;

pub const PTHREAD_STACK_MIN: ::size_t = 16384;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: ::c_int = 3;


pub const TIOCGSOFTCAR: ::c_ulong = 0x5419;
pub const TIOCSSOFTCAR: ::c_ulong = 0x541A;

pub const RLIMIT_NOFILE: ::c_int = 7;
pub const RLIMIT_NPROC: ::c_int = 6;

pub const O_APPEND: ::c_int = 1024;
pub const O_CREAT: ::c_int = 64;
pub const O_EXCL: ::c_int = 128;
pub const O_NOCTTY: ::c_int = 256;
pub const O_NONBLOCK: ::c_int = 2048;
pub const O_SYNC: ::c_int = 0x1000;
pub const O_RSYNC: ::c_int = 0x1000;
pub const O_DSYNC: ::c_int = 4096;
pub const O_FSYNC: ::c_int = 0x1000;
pub const O_NOATIME: ::c_int = 0o1000000;
pub const O_PATH: ::c_int = 0o10000000;
pub const O_TMPFILE: ::c_int = 0o20000000 | O_DIRECTORY;

pub const MAP_GROWSDOWN: ::c_int = 0x0100;

pub const EDEADLK: ::c_int = 35;
pub const ENAMETOOLONG: ::c_int = 36;
pub const ENOLCK: ::c_int = 37;
pub const ENOSYS: ::c_int = 38;
pub const ENOTEMPTY: ::c_int = 39;
pub const ELOOP: ::c_int = 40;
pub const ENOMSG: ::c_int = 42;
pub const EIDRM: ::c_int = 43;
pub const ECHRNG: ::c_int = 44;
pub const EL2NSYNC: ::c_int = 45;
pub const EL3HLT: ::c_int = 46;
pub const EL3RST: ::c_int = 47;
pub const ELNRNG: ::c_int = 48;
pub const EUNATCH: ::c_int = 49;
pub const ENOCSI: ::c_int = 50;
pub const EL2HLT: ::c_int = 51;
pub const EBADE: ::c_int = 52;
pub const EBADR: ::c_int = 53;
pub const EXFULL: ::c_int = 54;
pub const ENOANO: ::c_int = 55;
pub const EBADRQC: ::c_int = 56;
pub const EBADSLT: ::c_int = 57;
pub const EMULTIHOP: ::c_int = 72;
pub const EOVERFLOW: ::c_int = 75;
pub const ENOTUNIQ: ::c_int = 76;
pub const EBADFD: ::c_int = 77;
pub const EBADMSG: ::c_int = 74;
pub const EREMCHG: ::c_int = 78;
pub const ELIBACC: ::c_int = 79;
pub const ELIBBAD: ::c_int = 80;
pub const ELIBSCN: ::c_int = 81;
pub const ELIBMAX: ::c_int = 82;
pub const ELIBEXEC: ::c_int = 83;
pub const EILSEQ: ::c_int = 84;
pub const ERESTART: ::c_int = 85;
pub const ESTRPIPE: ::c_int = 86;
pub const EUSERS: ::c_int = 87;
pub const ENOTSOCK: ::c_int = 88;
pub const EDESTADDRREQ: ::c_int = 89;
pub const EMSGSIZE: ::c_int = 90;
pub const EPROTOTYPE: ::c_int = 91;
pub const ENOPROTOOPT: ::c_int = 92;
pub const EPROTONOSUPPORT: ::c_int = 93;
pub const ESOCKTNOSUPPORT: ::c_int = 94;
pub const EOPNOTSUPP: ::c_int = 95;
pub const EPFNOSUPPORT: ::c_int = 96;
pub const EAFNOSUPPORT: ::c_int = 97;
pub const EADDRINUSE: ::c_int = 98;
pub const EADDRNOTAVAIL: ::c_int = 99;
pub const ENETDOWN: ::c_int = 100;
pub const ENETUNREACH: ::c_int = 101;
pub const ENETRESET: ::c_int = 102;
pub const ECONNABORTED: ::c_int = 103;
pub const ECONNRESET: ::c_int = 104;
pub const ENOBUFS: ::c_int = 105;
pub const EISCONN: ::c_int = 106;
pub const ENOTCONN: ::c_int = 107;
pub const ESHUTDOWN: ::c_int = 108;
pub const ETOOMANYREFS: ::c_int = 109;
pub const ETIMEDOUT: ::c_int = 110;
pub const ECONNREFUSED: ::c_int = 111;
pub const EHOSTDOWN: ::c_int = 112;
pub const EHOSTUNREACH: ::c_int = 113;
pub const EALREADY: ::c_int = 114;
pub const EINPROGRESS: ::c_int = 115;
pub const ESTALE: ::c_int = 116;
pub const EDQUOT: ::c_int = 122;
pub const ENOMEDIUM: ::c_int = 123;
pub const EMEDIUMTYPE: ::c_int = 124;
pub const ECANCELED: ::c_int = 125;
pub const ENOKEY: ::c_int = 126;
pub const EKEYEXPIRED: ::c_int = 127;
pub const EKEYREVOKED: ::c_int = 128;
pub const EKEYREJECTED: ::c_int = 129;
pub const EOWNERDEAD: ::c_int = 130;
pub const ENOTRECOVERABLE: ::c_int = 131;
pub const EHWPOISON: ::c_int = 133;
pub const ERFKILL: ::c_int = 132;

pub const SOL_SOCKET: ::c_int = 1;

pub const SO_REUSEADDR: ::c_int = 2;
pub const SO_TYPE: ::c_int = 3;
pub const SO_ERROR: ::c_int = 4;
pub const SO_DONTROUTE: ::c_int = 5;
pub const SO_BROADCAST: ::c_int = 6;
pub const SO_SNDBUF: ::c_int = 7;
pub const SO_RCVBUF: ::c_int = 8;
pub const SO_KEEPALIVE: ::c_int = 9;
pub const SO_OOBINLINE: ::c_int = 10;
pub const SO_PRIORITY: ::c_int = 12;
pub const SO_LINGER: ::c_int = 13;
pub const SO_BSDCOMPAT: ::c_int = 14;
pub const SO_REUSEPORT: ::c_int = 15;
pub const SO_BINDTODEVICE: ::c_int = 25;
pub const SO_TIMESTAMP: ::c_int = 29;
pub const SO_ACCEPTCONN: ::c_int = 30;
pub const SO_MARK: ::c_int = 36;
pub const SO_PROTOCOL: ::c_int = 38;
pub const SO_DOMAIN: ::c_int = 39;
pub const SO_RXQ_OVFL: ::c_int = 40;
pub const SO_PEEK_OFF: ::c_int = 42;
pub const SO_BUSY_POLL: ::c_int = 46;

pub const SA_ONSTACK: ::c_int = 0x08000000;
pub const SA_SIGINFO: ::c_int = 0x00000004;
pub const SA_NOCLDWAIT: ::c_int = 0x00000002;

pub const SIGCHLD: ::c_int = 17;
pub const SIGBUS: ::c_int = 7;
pub const SIGUSR1: ::c_int = 10;
pub const SIGUSR2: ::c_int = 12;
pub const SIGCONT: ::c_int = 18;
pub const SIGSTOP: ::c_int = 19;
pub const SIGTSTP: ::c_int = 20;
pub const SIGURG: ::c_int = 23;
pub const SIGIO: ::c_int = 29;
pub const SIGSYS: ::c_int = 31;
pub const SIGSTKFLT: ::c_int = 16;
pub const SIGUNUSED: ::c_int = 31;
pub const SIGPOLL: ::c_int = 29;
pub const SIGPWR: ::c_int = 30;
pub const SIG_SETMASK: ::c_int = 2;
pub const SIG_BLOCK: ::c_int = 0x000000;
pub const SIG_UNBLOCK: ::c_int = 0x01;

pub const POLLWRNORM: ::c_short = 0x100;
pub const POLLWRBAND: ::c_short = 0x200;

pub const O_ASYNC: ::c_int = 0x2000;
pub const O_NDELAY: ::c_int = 0x800;

pub const PTRACE_DETACH: ::c_uint = 17;

pub const EFD_NONBLOCK: ::c_int = 0x800;

pub const F_GETLK: ::c_int = 5;
pub const F_GETOWN: ::c_int = 9;
pub const F_SETOWN: ::c_int = 8;
pub const F_SETLK: ::c_int = 6;
pub const F_SETLKW: ::c_int = 7;

pub const SFD_NONBLOCK: ::c_int = 0x0800;

pub const TIOCEXCL: ::c_ulong = 0x540C;
pub const TIOCNXCL: ::c_ulong = 0x540D;
pub const TIOCSCTTY: ::c_ulong = 0x540E;
pub const TIOCSTI: ::c_ulong = 0x5412;
pub const TIOCMGET: ::c_ulong = 0x5415;
pub const TIOCMBIS: ::c_ulong = 0x5416;
pub const TIOCMBIC: ::c_ulong = 0x5417;
pub const TIOCMSET: ::c_ulong = 0x5418;
pub const TIOCCONS: ::c_ulong = 0x541D;

pub const SFD_CLOEXEC: ::c_int = 0x080000;

pub const NCCS: usize = 32;

pub const O_TRUNC: ::c_int = 512;

pub const O_CLOEXEC: ::c_int = 0x80000;

pub const EBFONT: ::c_int = 59;
pub const ENOSTR: ::c_int = 60;
pub const ENODATA: ::c_int = 61;
pub const ETIME: ::c_int = 62;
pub const ENOSR: ::c_int = 63;
pub const ENONET: ::c_int = 64;
pub const ENOPKG: ::c_int = 65;
pub const EREMOTE: ::c_int = 66;
pub const ENOLINK: ::c_int = 67;
pub const EADV: ::c_int = 68;
pub const ESRMNT: ::c_int = 69;
pub const ECOMM: ::c_int = 70;
pub const EPROTO: ::c_int = 71;
pub const EDOTDOT: ::c_int = 73;

pub const SA_NODEFER: ::c_int = 0x40000000;
pub const SA_RESETHAND: ::c_int = 0x80000000;
pub const SA_RESTART: ::c_int = 0x10000000;
pub const SA_NOCLDSTOP: ::c_int = 0x00000001;

pub const EPOLL_CLOEXEC: ::c_int = 0x80000;

pub const EFD_CLOEXEC: ::c_int = 0x80000;

pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 24;
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 32;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 8;

#[cfg(target_endian = "little")]
pub const PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP: ::pthread_mutex_t =
    ::pthread_mutex_t {
        __align: [],
        size: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0,
        ],
    };
#[cfg(target_endian = "little")]
pub const PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP: ::pthread_mutex_t =
    ::pthread_mutex_t {
        __align: [],
        size: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0,
        ],
    };
#[cfg(target_endian = "little")]
pub const PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP: ::pthread_mutex_t =
    ::pthread_mutex_t {
        __align: [],
        size: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0,
        ],
    };
#[cfg(target_endian = "big")]
pub const PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP: ::pthread_mutex_t =
    ::pthread_mutex_t {
        __align: [],
        size: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
            0, 0,
        ],
    };
#[cfg(target_endian = "big")]
pub const PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP: ::pthread_mutex_t =
    ::pthread_mutex_t {
        __align: [],
        size: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
            0, 0,
        ],
    };
#[cfg(target_endian = "big")]
pub const PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP: ::pthread_mutex_t =
    ::pthread_mutex_t {
        __align: [],
        size: [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0,
            0, 0,
        ],
    };

pub const PTRACE_GETFPREGS: ::c_uint = 14;
pub const PTRACE_SETFPREGS: ::c_uint = 15;
pub const PTRACE_GETREGS: ::c_uint = 12;
pub const PTRACE_SETREGS: ::c_uint = 13;

