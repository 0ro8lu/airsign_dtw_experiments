# AirSign experimenter
This project contains the code responsible for generating graphs found in my thesis. It also contains the DTW algorithm implementation
which can be compiled to a dynamic library to be embedded in an Oculus Quest or any other android-based device. 

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Installation
Unfortunately i'm not going to upload any release files, so you're going to have to compile this from source. If you don't already have the rust
toolchain installed on your machine you can [Install it from here](https://www.rust-lang.org/tools/install) and arm yourself with a fresh copy.
Once you're all set just:

1. Clone the repo:
```bash
 git clone https://github.com/0ro8lu/airsign_dtw_experiments.git
```

2. Move to its root dir
```bash
 cd airsign_dtw_experiments
```

3. Compile :sunglasses:
```bash
 cargo build --release
```

## Usage
1. Running the experiments:
```bash
 cargo run --release -p experiments -- "/the/directory/where/you/have/the/siganture/data" "avg/min/max" "full/reduced"
```

## Contributing
If you like this project/found a bug/just wanna mess around, feel free to contribute ^^

1. Fork the repository.
2. Create a new branch: `git checkout -b feature-name`.
3. Make your changes.
4. Push your branch: `git push origin feature-name`.
5. Create a pull request.

## License
This project is licensed under the [MIT License](LICENSE).
