# Очередной RV64GC эмулятор

- Bazinga!

## Особенности

- [ ] Запуск riscv-pk
- [ ] Запуск ядра xv6
- [ ] Запуск ядра Linux
- [ ] Поддержка OpenSBI и Berkeley boot loader

## Поддерживаемые расширения

- [ ] RV32/64I
- [ ] RV32/64M
- [ ] RV32/64F
- [ ] RV32/64D
- [ ] RV32/64Q
- [ ] RV32/64V
- [ ] RV32/64A
- [ ] RV64C/32C
- [ ] RV32/64Zifencei
- [ ] RV32/64Zicsr
- [ ] SV48

## Сборка

```sh
git clone https://github.com/squeakbug/j4frv32emu
cd j4frv32emu
cargo build --release
```

## Запуск тестов

Зависимости:

- [riscv-gnu-toolchain](https://github.com/riscv/riscv-gnu-toolchain)
- [riscv-tests](https://github.com/riscv/riscv-tests)

```sh
cargo run --bin riscv-tests
```

## Ссылки

- [RISC-V ISA](https://riscv.org/specifications/)
- [Spike - Reference implementation](https://github.com/riscv-software-src/riscv-isa-sim)
- [JH7110 Datasheet (SoC)](https://starfivetech.com/uploads/JH7110.pdf)
- [E24 Datasheet (RISC-V Core)](https://sifive-china.oss-cn-zhangjiakou.aliyuncs.com/Standard%20Core%20IP/e24_core_complex_manual_21G2.pdf)
- [Virtio Device](https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html)
- [SiFive Interrupt Cookbook](https://sifive.cdn.prismic.io/sifive/0d163928-2128-42be-a75a-464df65e04e0_sifive-interrupt-cookbook.pdf)

-- 

https://github.com/chipsalliance/dromajo
