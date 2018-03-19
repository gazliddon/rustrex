define conn
    target remote localhost:6809
    restore asm/out/all.bin binary 0x9900
    set $pc=0x9900
end
document conn
Connecr to rustrex
end

