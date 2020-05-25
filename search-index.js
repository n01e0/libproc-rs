var searchIndex = JSON.parse('{\
"dmesg":{"doc":"A `dmesg` command that is a simple demonstration program…","i":[[5,"main","dmesg","",null,[[]]]],"p":[]},\
"libproc":{"doc":"`libproc` is a library for getting information about…","i":[[0,"libproc","libproc","`libproc` module provides library methods for getting…",null,null],[0,"proc_pid","libproc::libproc","Get basic information about processes by PID",null,null],[3,"ListThreads","libproc::libproc::proc_pid","",null,null],[4,"ProcType","","The `ProcType` type. Used to specify what type of…",null,null],[13,"ProcAllPIDS","","",0,null],[13,"ProcPGRPOnly","","",0,null],[13,"ProcTTYOnly","","",0,null],[13,"ProcUIDOnly","","",0,null],[13,"ProcRUIDOnly","","",0,null],[13,"ProcPPIDOnly","","",0,null],[4,"PidInfoFlavor","","An enum used to specify what type of information about a…",null,null],[13,"ListFDs","","",1,null],[13,"TaskAllInfo","","",1,null],[13,"TBSDInfo","","",1,null],[13,"TaskInfo","","",1,null],[13,"ThreadInfo","","",1,null],[13,"ListThreads","","",1,null],[13,"RegionInfo","","",1,null],[13,"RegionPathInfo","","",1,null],[13,"VNodePathInfo","","",1,null],[13,"ThreadPathInfo","","",1,null],[13,"PathInfo","","",1,null],[13,"WorkQueueInfo","","",1,null],[4,"PidInfo","","The `PidInfo` enum contains a piece of information about a…",null,null],[13,"ListFDs","","File Descriptors used by Process",2,null],[13,"TaskAllInfo","","",2,null],[13,"TBSDInfo","","",2,null],[13,"TaskInfo","","",2,null],[13,"ThreadInfo","","",2,null],[13,"ListThreads","","A list of Thread IDs",2,null],[13,"RegionInfo","","",2,null],[13,"RegionPathInfo","","",2,null],[13,"VNodePathInfo","","",2,null],[13,"ThreadPathInfo","","",2,null],[13,"PathInfo","","The path of the executable being run as the process",2,null],[13,"WorkQueueInfo","","",2,null],[5,"listpids","","Returns the PIDs of the processes active that match the…",null,[[["proctype",4]],[["string",3],["result",4],["vec",3]]]],[5,"pidinfo","","Get info about a process",null,[[],[["string",3],["pidinfo",8],["result",4]]]],[5,"regionfilename","","Get the filename associated with a memory region",null,[[],[["string",3],["result",4]]]],[5,"pidpath","","Get the path of the executable file being run for a process",null,[[],[["string",3],["result",4]]]],[5,"libversion","","Get the major and minor version numbers of the native…",null,[[],[["string",3],["result",4]]]],[5,"name","","Get the name of a process",null,[[],[["string",3],["result",4]]]],[5,"listpidinfo","","Get information on all running processes.",null,[[],[["string",3],["result",4],["vec",3]]]],[5,"pidcwd","","Gets the path of current working directory for the process…",null,[[["pid_t",6]],[["string",3],["result",4],["pathbuf",3]]]],[5,"cwdself","","Gets path of current working directory for the current…",null,[[],[["string",3],["result",4],["pathbuf",3]]]],[5,"am_root","","Determine if the current user ID of this process is root",null,[[]]],[8,"PIDInfo","","",null,null],[10,"flavor","","",3,[[],["pidinfoflavor",4]]],[8,"ListPIDInfo","","",null,null],[16,"Item","","",4,null],[10,"flavor","","",4,[[],["pidinfoflavor",4]]],[0,"kmesg_buffer","libproc::libproc","Read from the Kernel Message buffer",null,null],[5,"kmsgbuf","libproc::libproc::kmesg_buffer","Get upto buffersize bytes from the the kernel message…",null,[[],[["string",3],["result",4]]]],[0,"work_queue_info","libproc::libproc","Information about Work Queues - very MacOS specific",null,null],[3,"WorkQueueInfo","libproc::libproc::work_queue_info","",null,null],[12,"pwq_nthreads","","",5,null],[12,"pwq_runthreads","","",5,null],[12,"pwq_blockedthreads","","",5,null],[12,"reserved","","",5,null],[0,"thread_info","libproc::libproc","Information about Threads runing inside processes",null,null],[3,"ThreadInfo","libproc::libproc::thread_info","",null,null],[12,"pth_user_time","","",6,null],[12,"pth_system_time","","",6,null],[12,"pth_cpu_usage","","",6,null],[12,"pth_policy","","",6,null],[12,"pth_run_state","","",6,null],[12,"pth_flags","","",6,null],[12,"pth_sleep_time","","",6,null],[12,"pth_curpri","","",6,null],[12,"pth_priority","","",6,null],[12,"pth_maxpriority","","",6,null],[12,"pth_name","","",6,null],[0,"task_info","libproc::libproc","Information about Tasks - very MacOS specific",null,null],[3,"TaskInfo","libproc::libproc::task_info","",null,null],[12,"pti_virtual_size","","",7,null],[12,"pti_resident_size","","",7,null],[12,"pti_total_user","","",7,null],[12,"pti_total_system","","",7,null],[12,"pti_threads_user","","",7,null],[12,"pti_threads_system","","",7,null],[12,"pti_policy","","",7,null],[12,"pti_faults","","",7,null],[12,"pti_pageins","","",7,null],[12,"pti_cow_faults","","",7,null],[12,"pti_messages_sent","","",7,null],[12,"pti_messages_received","","",7,null],[12,"pti_syscalls_mach","","",7,null],[12,"pti_syscalls_unix","","",7,null],[12,"pti_csw","","",7,null],[12,"pti_threadnum","","",7,null],[12,"pti_numrunning","","",7,null],[12,"pti_priority","","",7,null],[3,"TaskAllInfo","","",null,null],[12,"pbsd","","",8,null],[12,"ptinfo","","",8,null],[0,"bsd_info","libproc::libproc","BSD specific information - very MacOS specific",null,null],[3,"BSDInfo","libproc::libproc::bsd_info","",null,null],[12,"pbi_flags","","",9,null],[12,"pbi_status","","",9,null],[12,"pbi_xstatus","","",9,null],[12,"pbi_pid","","",9,null],[12,"pbi_ppid","","",9,null],[12,"pbi_uid","","",9,null],[12,"pbi_gid","","",9,null],[12,"pbi_ruid","","",9,null],[12,"pbi_rgid","","",9,null],[12,"pbi_svuid","","",9,null],[12,"pbi_svgid","","",9,null],[12,"rfu_1","","",9,null],[12,"pbi_comm","","",9,null],[12,"pbi_name","","",9,null],[12,"pbi_nfiles","","",9,null],[12,"pbi_pgid","","",9,null],[12,"pbi_pjobc","","",9,null],[12,"e_tdev","","",9,null],[12,"e_tpgid","","",9,null],[12,"pbi_nice","","",9,null],[12,"pbi_start_tvsec","","",9,null],[12,"pbi_start_tvusec","","",9,null],[0,"pid_rusage","libproc::libproc","Information about Process Resource Usage",null,null],[3,"RUsageInfoV0","libproc::libproc::pid_rusage","",null,null],[12,"ri_uuid","","",10,null],[12,"ri_user_time","","",10,null],[12,"ri_system_time","","",10,null],[12,"ri_pkg_idle_wkups","","",10,null],[12,"ri_interrupt_wkups","","",10,null],[12,"ri_pageins","","",10,null],[12,"ri_wired_size","","",10,null],[12,"ri_resident_size","","",10,null],[12,"ri_phys_footprint","","",10,null],[12,"ri_proc_start_abstime","","",10,null],[12,"ri_proc_exit_abstime","","",10,null],[3,"RUsageInfoV1","","",null,null],[12,"ri_uuid","","",11,null],[12,"ri_user_time","","",11,null],[12,"ri_system_time","","",11,null],[12,"ri_pkg_idle_wkups","","",11,null],[12,"ri_interrupt_wkups","","",11,null],[12,"ri_pageins","","",11,null],[12,"ri_wired_size","","",11,null],[12,"ri_resident_size","","",11,null],[12,"ri_phys_footprint","","",11,null],[12,"ri_proc_start_abstime","","",11,null],[12,"ri_proc_exit_abstime","","",11,null],[12,"ri_child_user_time","","",11,null],[12,"ri_child_system_time","","",11,null],[12,"ri_child_pkg_idle_wkups","","",11,null],[12,"ri_child_interrupt_wkups","","",11,null],[12,"ri_child_pageins","","",11,null],[12,"ri_child_elapsed_abstime","","",11,null],[3,"RUsageInfoV2","","",null,null],[12,"ri_uuid","","",12,null],[12,"ri_user_time","","",12,null],[12,"ri_system_time","","",12,null],[12,"ri_pkg_idle_wkups","","",12,null],[12,"ri_interrupt_wkups","","",12,null],[12,"ri_pageins","","",12,null],[12,"ri_wired_size","","",12,null],[12,"ri_resident_size","","",12,null],[12,"ri_phys_footprint","","",12,null],[12,"ri_proc_start_abstime","","",12,null],[12,"ri_proc_exit_abstime","","",12,null],[12,"ri_child_user_time","","",12,null],[12,"ri_child_system_time","","",12,null],[12,"ri_child_pkg_idle_wkups","","",12,null],[12,"ri_child_interrupt_wkups","","",12,null],[12,"ri_child_pageins","","",12,null],[12,"ri_child_elapsed_abstime","","",12,null],[12,"ri_diskio_bytesread","","",12,null],[12,"ri_diskio_byteswritten","","",12,null],[3,"RUsageInfoV3","","",null,null],[12,"ri_uuid","","",13,null],[12,"ri_user_time","","",13,null],[12,"ri_system_time","","",13,null],[12,"ri_pkg_idle_wkups","","",13,null],[12,"ri_interrupt_wkups","","",13,null],[12,"ri_pageins","","",13,null],[12,"ri_wired_size","","",13,null],[12,"ri_resident_size","","",13,null],[12,"ri_phys_footprint","","",13,null],[12,"ri_proc_start_abstime","","",13,null],[12,"ri_proc_exit_abstime","","",13,null],[12,"ri_child_user_time","","",13,null],[12,"ri_child_system_time","","",13,null],[12,"ri_child_pkg_idle_wkups","","",13,null],[12,"ri_child_interrupt_wkups","","",13,null],[12,"ri_child_pageins","","",13,null],[12,"ri_child_elapsed_abstime","","",13,null],[12,"ri_diskio_bytesread","","",13,null],[12,"ri_diskio_byteswritten","","",13,null],[12,"ri_cpu_time_qos_default","","",13,null],[12,"ri_cpu_time_qos_maintenance","","",13,null],[12,"ri_cpu_time_qos_background","","",13,null],[12,"ri_cpu_time_qos_utility","","",13,null],[12,"ri_cpu_time_qos_legacy","","",13,null],[12,"ri_cpu_time_qos_user_initiated","","",13,null],[12,"ri_cpu_time_qos_user_interactive","","",13,null],[12,"ri_billed_system_time","","",13,null],[12,"ri_serviced_system_time","","",13,null],[3,"RUsageInfoV4","","",null,null],[12,"ri_uuid","","",14,null],[12,"ri_user_time","","",14,null],[12,"ri_system_time","","",14,null],[12,"ri_pkg_idle_wkups","","",14,null],[12,"ri_interrupt_wkups","","",14,null],[12,"ri_pageins","","",14,null],[12,"ri_wired_size","","",14,null],[12,"ri_resident_size","","",14,null],[12,"ri_phys_footprint","","",14,null],[12,"ri_proc_start_abstime","","",14,null],[12,"ri_proc_exit_abstime","","",14,null],[12,"ri_child_user_time","","",14,null],[12,"ri_child_system_time","","",14,null],[12,"ri_child_pkg_idle_wkups","","",14,null],[12,"ri_child_interrupt_wkups","","",14,null],[12,"ri_child_pageins","","",14,null],[12,"ri_child_elapsed_abstime","","",14,null],[12,"ri_diskio_bytesread","","",14,null],[12,"ri_diskio_byteswritten","","",14,null],[12,"ri_cpu_time_qos_default","","",14,null],[12,"ri_cpu_time_qos_maintenance","","",14,null],[12,"ri_cpu_time_qos_background","","",14,null],[12,"ri_cpu_time_qos_utility","","",14,null],[12,"ri_cpu_time_qos_legacy","","",14,null],[12,"ri_cpu_time_qos_user_initiated","","",14,null],[12,"ri_cpu_time_qos_user_interactive","","",14,null],[12,"ri_billed_system_time","","",14,null],[12,"ri_serviced_system_time","","",14,null],[12,"ri_logical_writes","","",14,null],[12,"ri_lifetime_max_phys_footprint","","",14,null],[12,"ri_instructions","","",14,null],[12,"ri_cycles","","",14,null],[12,"ri_billed_energy","","",14,null],[12,"ri_serviced_energy","","",14,null],[12,"ri_interval_max_phys_footprint","","",14,null],[12,"ri_unused","","",14,null],[4,"PidRUsageFlavor","","",null,null],[13,"V0","","",15,null],[13,"V1","","",15,null],[13,"V2","","",15,null],[13,"V3","","",15,null],[13,"V4","","",15,null],[5,"pidrusage","","Returns the information about resources of the process…",null,[[],[["string",3],["result",4],["pidrusage",8]]]],[8,"PIDRUsage","","",null,null],[10,"flavor","","",16,[[],["pidrusageflavor",4]]],[0,"file_info","libproc::libproc","Information about Files and File Descriptors used by…",null,null],[3,"ListFDs","libproc::libproc::file_info","",null,null],[3,"ProcFDInfo","","",null,null],[12,"proc_fd","","",17,null],[12,"proc_fdtype","","",17,null],[4,"PIDFDInfoFlavor","","",null,null],[13,"VNodeInfo","","",18,null],[13,"VNodePathInfo","","",18,null],[13,"SocketInfo","","",18,null],[13,"PSEMInfo","","",18,null],[13,"PSHMInfo","","",18,null],[13,"PipeInfo","","",18,null],[13,"KQueueInfo","","",18,null],[13,"ATalkInfo","","",18,null],[4,"ProcFDType","","",null,null],[13,"ATalk","","AppleTalk",19,null],[13,"VNode","","Vnode",19,null],[13,"Socket","","Socket",19,null],[13,"PSHM","","POSIX shared memory",19,null],[13,"PSEM","","POSIX semaphore",19,null],[13,"KQueue","","Kqueue",19,null],[13,"Pipe","","Pipe",19,null],[13,"FSEvents","","FSEvents",19,null],[13,"Unknown","","Unknown",19,null],[5,"pidfdinfo","","Returns the information about file descriptors of the…",null,[[],[["pidfdinfo",8],["string",3],["result",4]]]],[8,"PIDFDInfo","","",null,null],[10,"flavor","","",20,[[],["pidfdinfoflavor",4]]],[0,"net_info","libproc::libproc","Information about Network usage by a process",null,null],[3,"SocketFDInfo","libproc::libproc::net_info","",null,null],[12,"pfi","","",21,null],[12,"psi","","",21,null],[3,"ProcFileInfo","","",null,null],[12,"fi_openflags","","",22,null],[12,"fi_status","","",22,null],[12,"fi_offset","","",22,null],[12,"fi_type","","",22,null],[12,"rfu_1","","",22,null],[3,"SocketInfo","","",null,null],[12,"soi_stat","","",23,null],[12,"soi_so","","",23,null],[12,"soi_pcb","","",23,null],[12,"soi_type","","",23,null],[12,"soi_protocol","","",23,null],[12,"soi_family","","",23,null],[12,"soi_options","","",23,null],[12,"soi_linger","","",23,null],[12,"soi_state","","",23,null],[12,"soi_qlen","","",23,null],[12,"soi_incqlen","","",23,null],[12,"soi_qlimit","","",23,null],[12,"soi_timeo","","",23,null],[12,"soi_error","","",23,null],[12,"soi_oobmark","","",23,null],[12,"soi_rcv","","",23,null],[12,"soi_snd","","",23,null],[12,"soi_kind","","",23,null],[12,"rfu_1","","",23,null],[12,"soi_proto","","",23,null],[3,"VInfoStat","","",null,null],[12,"vst_dev","","",24,null],[12,"vst_mode","","",24,null],[12,"vst_nlink","","",24,null],[12,"vst_ino","","",24,null],[12,"vst_uid","","",24,null],[12,"vst_gid","","",24,null],[12,"vst_atime","","",24,null],[12,"vst_atimensec","","",24,null],[12,"vst_mtime","","",24,null],[12,"vst_mtimensec","","",24,null],[12,"vst_ctime","","",24,null],[12,"vst_ctimensec","","",24,null],[12,"vst_birthtime","","",24,null],[12,"vst_birthtimensec","","",24,null],[12,"vst_size","","",24,null],[12,"vst_blocks","","",24,null],[12,"vst_blksize","","",24,null],[12,"vst_flags","","",24,null],[12,"vst_gen","","",24,null],[12,"vst_rdev","","",24,null],[12,"vst_qspare","","",24,null],[3,"SockBufInfo","","",null,null],[12,"sbi_cc","","",25,null],[12,"sbi_hiwat","","",25,null],[12,"sbi_mbcnt","","",25,null],[12,"sbi_mbmax","","",25,null],[12,"sbi_lowat","","",25,null],[12,"sbi_flags","","",25,null],[12,"sbi_timeo","","",25,null],[3,"In4In6Addr","","",null,null],[12,"i46a_pad32","","",26,null],[12,"i46a_addr4","","",26,null],[3,"InSockInfo","","",null,null],[12,"insi_fport","","",27,null],[12,"insi_lport","","",27,null],[12,"insi_gencnt","","",27,null],[12,"insi_flags","","",27,null],[12,"insi_flow","","",27,null],[12,"insi_vflag","","",27,null],[12,"insi_ip_ttl","","",27,null],[12,"rfu_1","","",27,null],[12,"insi_faddr","","",27,null],[12,"insi_laddr","","",27,null],[12,"insi_v4","","",27,null],[12,"insi_v6","","",27,null],[3,"InSIV4","","",null,null],[12,"in4_top","","",28,null],[3,"InSIV6","","",null,null],[12,"in6_hlim","","",29,null],[12,"in6_cksum","","",29,null],[12,"in6_ifindex","","",29,null],[12,"in6_hops","","",29,null],[3,"TcpSockInfo","","",null,null],[12,"tcpsi_ini","","",30,null],[12,"tcpsi_state","","",30,null],[12,"tcpsi_timer","","",30,null],[12,"tcpsi_mss","","",30,null],[12,"tcpsi_flags","","",30,null],[12,"rfu_1","","",30,null],[12,"tcpsi_tp","","",30,null],[3,"UnSockInfo","","",null,null],[12,"unsi_conn_so","","",31,null],[12,"unsi_conn_pcb","","",31,null],[12,"unsi_addr","","",31,null],[12,"unsi_caddr","","",31,null],[3,"NdrvInfo","","",null,null],[12,"ndrvsi_if_family","","",32,null],[12,"ndrvsi_if_unit","","",32,null],[12,"ndrvsi_if_name","","",32,null],[3,"KernEventInfo","","",null,null],[12,"kesi_vendor_code_filter","","",33,null],[12,"kesi_class_filter","","",33,null],[12,"kesi_subclass_filter","","",33,null],[3,"KernCtlInfo","","",null,null],[12,"kcsi_id","","",34,null],[12,"kcsi_reg_unit","","",34,null],[12,"kcsi_flags","","",34,null],[12,"kcsi_recvbufsize","","",34,null],[12,"kcsi_sendbufsize","","",34,null],[12,"kcsi_unit","","",34,null],[12,"kcsi_name","","",34,null],[19,"SocketInfoProto","","",null,null],[12,"pri_in","","",35,null],[12,"pri_tcp","","",35,null],[12,"pri_un","","",35,null],[12,"pri_ndrv","","",35,null],[12,"pri_kern_event","","",35,null],[12,"pri_kern_ctl","","",35,null],[19,"InSIAddr","","",null,null],[12,"ina_46","","",36,null],[12,"ina_6","","",36,null],[19,"UnSIAddr","","",null,null],[12,"ua_sun","","",37,null],[12,"ua_dummy","","",37,null],[4,"SocketInfoKind","","",null,null],[13,"Generic","","",38,null],[13,"In","","IPv4 and IPv6 Sockets",38,null],[13,"Tcp","","TCP Sockets",38,null],[13,"Un","","Unix Domain Sockets",38,null],[13,"Ndrv","","PF_NDRV Sockets",38,null],[13,"KernEvent","","Kernel Event Sockets",38,null],[13,"KernCtl","","Kernel Control Sockets",38,null],[13,"Unknown","","Unknown",38,null],[4,"TcpSIState","","",null,null],[13,"Closed","","Closed",39,null],[13,"Listen","","Listening for connection",39,null],[13,"SynSent","","Active, have sent syn",39,null],[13,"SynReceived","","Have send and received syn",39,null],[13,"Established","","Established",39,null],[13,"CloseWait","","Rcvd fin, waiting for close",39,null],[13,"FinWait1","","Have closed, sent fin",39,null],[13,"Closing","","Closed xchd FIN; await FIN ACK",39,null],[13,"LastAck","","Had fin and close; await FIN ACK",39,null],[13,"FinWait2","","Have closed, fin is acked",39,null],[13,"TimeWait","","In 2*msl quiet wait after close",39,null],[13,"Reserved","","Pseudo state: reserved",39,null],[13,"Unknown","","Unknown",39,null],[11,"from","libproc::libproc::proc_pid","",40,[[]]],[11,"into","","",40,[[]]],[11,"borrow","","",40,[[]]],[11,"try_from","","",40,[[],["result",4]]],[11,"try_into","","",40,[[],["result",4]]],[11,"borrow_mut","","",40,[[]]],[11,"type_id","","",40,[[],["typeid",3]]],[11,"from","","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"borrow_mut","","",0,[[]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"borrow_mut","","",1,[[]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"borrow_mut","","",2,[[]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","libproc::libproc::work_queue_info","",5,[[]]],[11,"into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"borrow_mut","","",5,[[]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","libproc::libproc::thread_info","",6,[[]]],[11,"into","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"borrow_mut","","",6,[[]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","libproc::libproc::task_info","",7,[[]]],[11,"into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"borrow_mut","","",7,[[]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","","",8,[[]]],[11,"into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"borrow_mut","","",8,[[]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","libproc::libproc::bsd_info","",9,[[]]],[11,"into","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"borrow_mut","","",9,[[]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"from","libproc::libproc::pid_rusage","",10,[[]]],[11,"into","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"borrow_mut","","",10,[[]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"borrow_mut","","",11,[[]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"from","","",12,[[]]],[11,"into","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"borrow_mut","","",12,[[]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"from","","",13,[[]]],[11,"into","","",13,[[]]],[11,"borrow","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"try_into","","",13,[[],["result",4]]],[11,"borrow_mut","","",13,[[]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"from","","",14,[[]]],[11,"into","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"borrow_mut","","",14,[[]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"from","","",15,[[]]],[11,"into","","",15,[[]]],[11,"borrow","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"try_into","","",15,[[],["result",4]]],[11,"borrow_mut","","",15,[[]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"from","libproc::libproc::file_info","",41,[[]]],[11,"into","","",41,[[]]],[11,"borrow","","",41,[[]]],[11,"try_from","","",41,[[],["result",4]]],[11,"try_into","","",41,[[],["result",4]]],[11,"borrow_mut","","",41,[[]]],[11,"type_id","","",41,[[],["typeid",3]]],[11,"from","","",17,[[]]],[11,"into","","",17,[[]]],[11,"borrow","","",17,[[]]],[11,"try_from","","",17,[[],["result",4]]],[11,"try_into","","",17,[[],["result",4]]],[11,"borrow_mut","","",17,[[]]],[11,"type_id","","",17,[[],["typeid",3]]],[11,"from","","",18,[[]]],[11,"into","","",18,[[]]],[11,"borrow","","",18,[[]]],[11,"try_from","","",18,[[],["result",4]]],[11,"try_into","","",18,[[],["result",4]]],[11,"borrow_mut","","",18,[[]]],[11,"type_id","","",18,[[],["typeid",3]]],[11,"from","","",19,[[]]],[11,"into","","",19,[[]]],[11,"to_owned","","",19,[[]]],[11,"clone_into","","",19,[[]]],[11,"borrow","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"try_into","","",19,[[],["result",4]]],[11,"borrow_mut","","",19,[[]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"from","libproc::libproc::net_info","",21,[[]]],[11,"into","","",21,[[]]],[11,"borrow","","",21,[[]]],[11,"try_from","","",21,[[],["result",4]]],[11,"try_into","","",21,[[],["result",4]]],[11,"borrow_mut","","",21,[[]]],[11,"type_id","","",21,[[],["typeid",3]]],[11,"from","","",22,[[]]],[11,"into","","",22,[[]]],[11,"borrow","","",22,[[]]],[11,"try_from","","",22,[[],["result",4]]],[11,"try_into","","",22,[[],["result",4]]],[11,"borrow_mut","","",22,[[]]],[11,"type_id","","",22,[[],["typeid",3]]],[11,"from","","",23,[[]]],[11,"into","","",23,[[]]],[11,"borrow","","",23,[[]]],[11,"try_from","","",23,[[],["result",4]]],[11,"try_into","","",23,[[],["result",4]]],[11,"borrow_mut","","",23,[[]]],[11,"type_id","","",23,[[],["typeid",3]]],[11,"from","","",24,[[]]],[11,"into","","",24,[[]]],[11,"borrow","","",24,[[]]],[11,"try_from","","",24,[[],["result",4]]],[11,"try_into","","",24,[[],["result",4]]],[11,"borrow_mut","","",24,[[]]],[11,"type_id","","",24,[[],["typeid",3]]],[11,"from","","",25,[[]]],[11,"into","","",25,[[]]],[11,"borrow","","",25,[[]]],[11,"try_from","","",25,[[],["result",4]]],[11,"try_into","","",25,[[],["result",4]]],[11,"borrow_mut","","",25,[[]]],[11,"type_id","","",25,[[],["typeid",3]]],[11,"from","","",26,[[]]],[11,"into","","",26,[[]]],[11,"to_owned","","",26,[[]]],[11,"clone_into","","",26,[[]]],[11,"borrow","","",26,[[]]],[11,"try_from","","",26,[[],["result",4]]],[11,"try_into","","",26,[[],["result",4]]],[11,"borrow_mut","","",26,[[]]],[11,"type_id","","",26,[[],["typeid",3]]],[11,"from","","",27,[[]]],[11,"into","","",27,[[]]],[11,"to_owned","","",27,[[]]],[11,"clone_into","","",27,[[]]],[11,"borrow","","",27,[[]]],[11,"try_from","","",27,[[],["result",4]]],[11,"try_into","","",27,[[],["result",4]]],[11,"borrow_mut","","",27,[[]]],[11,"type_id","","",27,[[],["typeid",3]]],[11,"from","","",28,[[]]],[11,"into","","",28,[[]]],[11,"to_owned","","",28,[[]]],[11,"clone_into","","",28,[[]]],[11,"borrow","","",28,[[]]],[11,"try_from","","",28,[[],["result",4]]],[11,"try_into","","",28,[[],["result",4]]],[11,"borrow_mut","","",28,[[]]],[11,"type_id","","",28,[[],["typeid",3]]],[11,"from","","",29,[[]]],[11,"into","","",29,[[]]],[11,"to_owned","","",29,[[]]],[11,"clone_into","","",29,[[]]],[11,"borrow","","",29,[[]]],[11,"try_from","","",29,[[],["result",4]]],[11,"try_into","","",29,[[],["result",4]]],[11,"borrow_mut","","",29,[[]]],[11,"type_id","","",29,[[],["typeid",3]]],[11,"from","","",30,[[]]],[11,"into","","",30,[[]]],[11,"to_owned","","",30,[[]]],[11,"clone_into","","",30,[[]]],[11,"borrow","","",30,[[]]],[11,"try_from","","",30,[[],["result",4]]],[11,"try_into","","",30,[[],["result",4]]],[11,"borrow_mut","","",30,[[]]],[11,"type_id","","",30,[[],["typeid",3]]],[11,"from","","",31,[[]]],[11,"into","","",31,[[]]],[11,"to_owned","","",31,[[]]],[11,"clone_into","","",31,[[]]],[11,"borrow","","",31,[[]]],[11,"try_from","","",31,[[],["result",4]]],[11,"try_into","","",31,[[],["result",4]]],[11,"borrow_mut","","",31,[[]]],[11,"type_id","","",31,[[],["typeid",3]]],[11,"from","","",32,[[]]],[11,"into","","",32,[[]]],[11,"to_owned","","",32,[[]]],[11,"clone_into","","",32,[[]]],[11,"borrow","","",32,[[]]],[11,"try_from","","",32,[[],["result",4]]],[11,"try_into","","",32,[[],["result",4]]],[11,"borrow_mut","","",32,[[]]],[11,"type_id","","",32,[[],["typeid",3]]],[11,"from","","",33,[[]]],[11,"into","","",33,[[]]],[11,"to_owned","","",33,[[]]],[11,"clone_into","","",33,[[]]],[11,"borrow","","",33,[[]]],[11,"try_from","","",33,[[],["result",4]]],[11,"try_into","","",33,[[],["result",4]]],[11,"borrow_mut","","",33,[[]]],[11,"type_id","","",33,[[],["typeid",3]]],[11,"from","","",34,[[]]],[11,"into","","",34,[[]]],[11,"to_owned","","",34,[[]]],[11,"clone_into","","",34,[[]]],[11,"borrow","","",34,[[]]],[11,"try_from","","",34,[[],["result",4]]],[11,"try_into","","",34,[[],["result",4]]],[11,"borrow_mut","","",34,[[]]],[11,"type_id","","",34,[[],["typeid",3]]],[11,"from","","",35,[[]]],[11,"into","","",35,[[]]],[11,"borrow","","",35,[[]]],[11,"try_from","","",35,[[],["result",4]]],[11,"try_into","","",35,[[],["result",4]]],[11,"borrow_mut","","",35,[[]]],[11,"type_id","","",35,[[],["typeid",3]]],[11,"from","","",36,[[]]],[11,"into","","",36,[[]]],[11,"to_owned","","",36,[[]]],[11,"clone_into","","",36,[[]]],[11,"borrow","","",36,[[]]],[11,"try_from","","",36,[[],["result",4]]],[11,"try_into","","",36,[[],["result",4]]],[11,"borrow_mut","","",36,[[]]],[11,"type_id","","",36,[[],["typeid",3]]],[11,"from","","",37,[[]]],[11,"into","","",37,[[]]],[11,"to_owned","","",37,[[]]],[11,"clone_into","","",37,[[]]],[11,"borrow","","",37,[[]]],[11,"try_from","","",37,[[],["result",4]]],[11,"try_into","","",37,[[],["result",4]]],[11,"borrow_mut","","",37,[[]]],[11,"type_id","","",37,[[],["typeid",3]]],[11,"from","","",38,[[]]],[11,"into","","",38,[[]]],[11,"to_owned","","",38,[[]]],[11,"clone_into","","",38,[[]]],[11,"borrow","","",38,[[]]],[11,"try_from","","",38,[[],["result",4]]],[11,"try_into","","",38,[[],["result",4]]],[11,"borrow_mut","","",38,[[]]],[11,"type_id","","",38,[[],["typeid",3]]],[11,"from","","",39,[[]]],[11,"into","","",39,[[]]],[11,"to_owned","","",39,[[]]],[11,"clone_into","","",39,[[]]],[11,"borrow","","",39,[[]]],[11,"try_from","","",39,[[],["result",4]]],[11,"try_into","","",39,[[],["result",4]]],[11,"borrow_mut","","",39,[[]]],[11,"type_id","","",39,[[],["typeid",3]]],[11,"flavor","libproc::libproc::work_queue_info","",5,[[],["pidinfoflavor",4]]],[11,"flavor","libproc::libproc::thread_info","",6,[[],["pidinfoflavor",4]]],[11,"flavor","libproc::libproc::task_info","",7,[[],["pidinfoflavor",4]]],[11,"flavor","","",8,[[],["pidinfoflavor",4]]],[11,"flavor","libproc::libproc::bsd_info","",9,[[],["pidinfoflavor",4]]],[11,"flavor","libproc::libproc::proc_pid","",40,[[],["pidinfoflavor",4]]],[11,"flavor","libproc::libproc::file_info","",41,[[],["pidinfoflavor",4]]],[11,"flavor","libproc::libproc::pid_rusage","",10,[[],["pidrusageflavor",4]]],[11,"flavor","","",11,[[],["pidrusageflavor",4]]],[11,"flavor","","",12,[[],["pidrusageflavor",4]]],[11,"flavor","","",13,[[],["pidrusageflavor",4]]],[11,"flavor","","",14,[[],["pidrusageflavor",4]]],[11,"flavor","libproc::libproc::net_info","",21,[[],["pidfdinfoflavor",4]]],[11,"from","libproc::libproc::file_info","",19,[[],["procfdtype",4]]],[11,"from","libproc::libproc::net_info","",38,[[["c_int",6]],["socketinfokind",4]]],[11,"from","","",39,[[["c_int",6]],["tcpsistate",4]]],[11,"clone","libproc::libproc::proc_pid","",0,[[],["proctype",4]]],[11,"clone","libproc::libproc::file_info","",19,[[],["procfdtype",4]]],[11,"clone","libproc::libproc::net_info","",38,[[],["socketinfokind",4]]],[11,"clone","","",26,[[],["in4in6addr",3]]],[11,"clone","","",27,[[],["insockinfo",3]]],[11,"clone","","",28,[[],["insiv4",3]]],[11,"clone","","",29,[[],["insiv6",3]]],[11,"clone","","",36,[[],["insiaddr",19]]],[11,"clone","","",39,[[],["tcpsistate",4]]],[11,"clone","","",30,[[],["tcpsockinfo",3]]],[11,"clone","","",31,[[],["unsockinfo",3]]],[11,"clone","","",37,[[],["unsiaddr",19]]],[11,"clone","","",32,[[],["ndrvinfo",3]]],[11,"clone","","",33,[[],["kerneventinfo",3]]],[11,"clone","","",34,[[],["kernctlinfo",3]]],[11,"default","libproc::libproc::work_queue_info","",5,[[],["workqueueinfo",3]]],[11,"default","libproc::libproc::thread_info","",6,[[],["threadinfo",3]]],[11,"default","libproc::libproc::task_info","",7,[[],["taskinfo",3]]],[11,"default","","",8,[[],["taskallinfo",3]]],[11,"default","libproc::libproc::bsd_info","",9,[[],["bsdinfo",3]]],[11,"default","libproc::libproc::pid_rusage","",10,[[],["rusageinfov0",3]]],[11,"default","","",11,[[],["rusageinfov1",3]]],[11,"default","","",12,[[],["rusageinfov2",3]]],[11,"default","","",13,[[],["rusageinfov3",3]]],[11,"default","","",14,[[],["rusageinfov4",3]]],[11,"default","libproc::libproc::net_info","",21,[[],["socketfdinfo",3]]],[11,"default","","",22,[[],["procfileinfo",3]]],[11,"default","","",23,[[],["socketinfo",3]]],[11,"default","","",24,[[],["vinfostat",3]]],[11,"default","","",25,[[],["sockbufinfo",3]]],[11,"default","","",35,[[],["socketinfoproto",19]]],[11,"default","","",26,[[],["in4in6addr",3]]],[11,"default","","",27,[[],["insockinfo",3]]],[11,"default","","",28,[[],["insiv4",3]]],[11,"default","","",29,[[],["insiv6",3]]],[11,"default","","",36,[[],["insiaddr",19]]],[11,"default","","",30,[[],["tcpsockinfo",3]]],[11,"default","","",31,[[],["unsockinfo",3]]],[11,"default","","",37,[[],["unsiaddr",19]]],[11,"default","","",32,[[],["ndrvinfo",3]]],[11,"default","","",33,[[],["kerneventinfo",3]]],[11,"default","","",34,[[],["kernctlinfo",3]]],[11,"fmt","libproc::libproc::pid_rusage","",12,[[["formatter",3]],["result",6]]],[11,"fmt","libproc::libproc::file_info","",19,[[["formatter",3]],["result",6]]],[11,"fmt","libproc::libproc::net_info","",38,[[["formatter",3]],["result",6]]],[11,"fmt","","",39,[[["formatter",3]],["result",6]]]],"p":[[4,"ProcType"],[4,"PidInfoFlavor"],[4,"PidInfo"],[8,"PIDInfo"],[8,"ListPIDInfo"],[3,"WorkQueueInfo"],[3,"ThreadInfo"],[3,"TaskInfo"],[3,"TaskAllInfo"],[3,"BSDInfo"],[3,"RUsageInfoV0"],[3,"RUsageInfoV1"],[3,"RUsageInfoV2"],[3,"RUsageInfoV3"],[3,"RUsageInfoV4"],[4,"PidRUsageFlavor"],[8,"PIDRUsage"],[3,"ProcFDInfo"],[4,"PIDFDInfoFlavor"],[4,"ProcFDType"],[8,"PIDFDInfo"],[3,"SocketFDInfo"],[3,"ProcFileInfo"],[3,"SocketInfo"],[3,"VInfoStat"],[3,"SockBufInfo"],[3,"In4In6Addr"],[3,"InSockInfo"],[3,"InSIV4"],[3,"InSIV6"],[3,"TcpSockInfo"],[3,"UnSockInfo"],[3,"NdrvInfo"],[3,"KernEventInfo"],[3,"KernCtlInfo"],[19,"SocketInfoProto"],[19,"InSIAddr"],[19,"UnSIAddr"],[4,"SocketInfoKind"],[4,"TcpSIState"],[3,"ListThreads"],[3,"ListFDs"]]},\
"procinfo":{"doc":"`procinfo` is a simple program to demonstrate the use of…","i":[[5,"getpid","procinfo","",null,[[]]],[5,"procinfo","","",null,[[]]],[5,"main","","",null,[[]]],[0,"c","","",null,null],[5,"getpid","procinfo::c","",null,null]],"p":[]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);