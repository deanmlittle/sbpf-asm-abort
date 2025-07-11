# sbpf-asm-abort

A 352 byte emergency abort program. Shut down any upgradable program in the case of an emergency and fail all transactions that invoke it until updated with a fix. Deployable in a single transaction with zero protocol changes. You can just do things.

Created with [sbpf](https://github.com/deanmlittle/sbpf)