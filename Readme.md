# audios21

Use your sound card to characterize various audio devices. This can be used for
benchmarking or pass/fail testing.

## Types of Measurement

- [ ] S21 Insertion Loss (Generate Bode plot with phase information)
- [ ] Time-domain delay information (attack/release times of dynamic range compressors)
- [ ] Compression graph (comparing input volume to output volume)

## Other Features

- [ ] Calibration
- [ ] Spoof device output via .wav file (i.e. for comparison against ffmpeg filters)
- [ ] Generate sweeps/tests based on JSON config file
- [ ] Generate graph visualizations based on JSON config file

## Origin of the Name

With the intention being to take Network Analyzer style S21 measurements of
Audio systems, the name was AudioS21. But cargo highlighted that a more
idiomatic package name would be "audios21" which sounds a bit like adios-21.

