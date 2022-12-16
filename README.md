<!-- PROJECT LOGO -->
<br />
<p align="center">
  <!-- <a href="https://github.com/othneildrew/Best-README-Template">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a> -->

  <h2 align="center">ZoKrates-api</h3>

  <p align="center">
    The ZoKrates core library wrap around an API
    <br />
    <!-- <a href="https://github.com/ZoKratesPlus/zokrates-api/issues"><strong>Explore the docs »</strong></a>
    <br /> -->
    <br />
    <!-- <a href="https://github.com/othneildrew/Best-README-Template">View Demo</a>
    · -->
    <a href="https://github.com/ZoKratesPlus/zokrates-api/issues">Report Bug</a>
    ·
    <a href="https://github.com/ZoKratesPlus/zokrates-api/issues">Request Feature</a>
  </p>
</p>


[![Generic badge](https://img.shields.io/badge/ZoKrates-0.8.3-yellow.svg)](https://github.com/Zokrates/ZoKrates/tree/0.8.3)
[![Generic badge](https://img.shields.io/badge/rocket.rs-0.5.0%E2%80%93rc.1-red.svg)](https://github.com/Zokrates/ZoKrates/tree/0.8.3)
[![License: LGPL v3](https://img.shields.io/badge/License-LGPL_v3-blue.svg)](https://www.gnu.org/licenses/lgpl-3.0)


<!-- TABLE OF CONTENTS -->
# Table of Contents

- [Table of Contents](#table-of-contents)
- [About The Project](#about-the-project)
  - [Setup](#setup)
    - [Docker](#docker)
    - [From source](#from-source)
      - [Docker](#docker-1)
      - [locally](#locally)
  - [Usage](#usage)
- [Contributing](#contributing)
  - [Running testing](#running-testing)
  - [Linting and Formatting](#linting-and-formatting)
    - [Built With](#built-with)
  - [License](#license)
  - [Acknowledgements](#acknowledgements)


# About The Project

<!-- [![Product Name Screen Shot][product-screenshot]](https://example.com) -->

ZoKrates-api wraps ZoKrates commands around an API in order to facilitate the generation of zk-SNARKs proofs on a large scale.

Here's why:
* Once you figure out how zk-SNARKs can solve your specific problem, Don't waste your time how to embed it on your system. Pull the image and communicate with the service through standard http requests.
* Though to be used as a micro-service, ZoKrates-api can be scaled horizontally and vertically with ease.
* Zokrates-api is written in rust and provides a performance boost compare to ZoKrates-js 

## Setup 

### Docker 
```sh
$ docker run alvaround/zokrates-api:latest
```

### From source
#### Docker

```bash
$ docker-compose up
```

#### locally
```bash
$ RUST_MIN_STACK=1000000 cargo +nightly run
```
Or with hot-loading:
```bash
$ RUST_MIN_STACK=1000000 cargo +nightly watch -x run
```

<!-- USAGE EXAMPLES -->
## Usage

Once you have the image running locally, open the following OpenAPI interface on this [link](http://0.0.0.0:8000/docs/index.html) in your browser.

# Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Open an Issue to discuss the problem at hand and possible solutions
2. Fork the Project
3. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
4. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
5. Push to the Branch (`git push origin feature/AmazingFeature`)
6. Open a Pull Request
## Running testing

Run tests:
```bash
$ cargo test
```

## Linting and Formatting
Rust format:
```bash
$ cargo fmt
```

Rust lint:
```bash
$ cargo clippy
```

### Built With

* [ZoKrates](https://zokrates.github.io/introduction.html)
* [Rocket](https://rocket.rs/)
* [Rocket-Okapi](https://docs.rs/rocket_okapi/latest/rocket_okapi/)

<!-- LICENSE -->
## License

Distributed under the GNU LGPL 3.0. See `LICENSE` for more information.

<!-- CONTACT -->
<!-- ## Contact

Your Name - [@your_twitter](https://twitter.com/your_username) - email@example.com

Project Link: [https://github.com/your_username/repo_name](https://github.com/your_username/repo_name) -->


<!-- ACKNOWLEDGEMENTS -->
## Acknowledgements
* [Best README Template](https://github.com/othneildrew/Best-README-Template)
