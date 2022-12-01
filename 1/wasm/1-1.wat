(module
    (import "runner" "calories" (func $calories (result i32)))
    (import "runner" "elf" (func $elf (result i32)))
    (func $readelf (result i32)
        (local $count i32)
        (local $intermediate i32)
        (loop $count_loop (result i32)
            call $calories
            local.tee $intermediate
            i32.eqz
            (if
                (then 
                    local.get $count
                    return
                )
                (else
                    local.get $count
                    local.get $intermediate
                    i32.add
                    local.set $count
                  )
            )
            br $count_loop
       )
    )
    (func $main (result i32)
        (local $max i32)
        (local $intermediate i32)
        (loop $elf_loop (result i32)
              call $elf
              i32.eqz
              (if
                (then
                    call $readelf
                    local.tee $intermediate
                    local.get $max
                    i32.gt_u
                    (if
                        (then
                            local.get $intermediate
                            local.set $max
                        )
                    )
                    br $elf_loop
                )
            )
            local.get $max
            return
        )
    )
    (export "run" (func $main))
)
