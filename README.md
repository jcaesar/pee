# Pee
> *P*ut bytes into a file. A bit like tee, but without the output.

Mostly made for [webassembly.sh](https://webassembly.sh/?run-command=pee%20-h).

Can also be useful if you don't like constructs like 

```bash
echo 42 | sudo tee /proc/sys/... >/dev/null
# or
sudo bash -c 'echo 42 >/proc/sys/...'
```

Example:
```bash
echo please sit down when you | sudo pee /dev/kmsg
sudo pee /dev/kmesg convenience arguments
sudo dmesg -T | tail
```
