# Thread vs Process in linux
## similarity
* They are all **task** in the kernel

## Difference
### Creation of process
call **fork** and in `copy_process`, the following structs will be copied and has its own struct for the process
    * files_struct
    * fs_struct
    * sighand_struct
    * signal_struct
    * mm_struct

### Creation of thread
call **pthread_create** and in `copy_process`, the following structs is not copied and only the reference count increments by 1. So we say the structs are shared between thread 
    * files_struct
    * fs_struct
    * sighand_struct
    * signal_struct
    * mm_struct