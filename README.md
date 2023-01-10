<!-- PROJECT LOGO -->
<br />
<p align="center">
  <!-- <a href="https://github.com/othneildrew/Best-README-Template">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a> -->

  <h2 align="center">ZoKrates-API</h3>

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
    - [Proof Setup](#proof-setup)
    - [Proof Generation](#proof-generation)
- [Contributing](#contributing)
  - [Running testing](#running-testing)
  - [Linting and Formatting](#linting-and-formatting)
    - [Built With](#built-with)
  - [License](#license)
  - [Acknowledgements](#acknowledgements)


# About The Project

<!-- [![Product Name Screen Shot][product-screenshot]](https://example.com) -->

ZoKrates-API is a JSON API wrapper around the Zokrates commands involved in the  generation of zk-SNARKs proofs.
Being able to perform Zokrates operations through http facilitates the automation of processes in which the calculation of zk-SNARKs is required.
Furthermore, ZoKrates-API was developed to be easily deployed as micro-service on the cloud, to avoid running into memory problems so common when developing zk-SNARKs.
With ZoKrates-API, you can have multiple instances running in parallel on a Kubernetes cluster.

ZoKrates-API aims to tackle the following problems:
* Once you figure out how zk-SNARKs can solve your specific problem, Don't waste your time how to embed it on your system. Pull the image and communicate with the service through standard http requests.
* Though to be used as a micro-service, ZoKrates-API can be scaled horizontally and vertically with ease.
* ZoKrates-API is written in rust and provides a performance boost and memory granularity compare to ZoKrates-js 

## Setup 

### Docker 
```sh
$ docker run -p 8000:8000 --env ROCKET_LOG_LEVEL=normal alvaround/zokrates-api:latest
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

Once you have the image running locally, open the OpenAPI interface on [http://0.0.0.0:8000/docs/index.html](http://0.0.0.0:8000/docs/index.html) in your browser.
In the url you can access all the methods support, endpoints, body schemas, and response tes supported by the API.

The API is able to generate proofs for multiple `.zok` at once. However, for each new `.zok` the Proof Setup process has to be done once.

### Proof Setup

  1. Add your `.zok` file in the `POST /compile` endpoint and click execute. Zokrates will compile the program on the server. This can take some time. If the program compiles successfully, the response constains the `program_hash` and the abi.json structure. You will need both information on later steps.
  2. Call the `POST /<PROGRAM_HASH>/proving-key` with the `program_hash` from previous step and the proving key of the program attached in the response.

### Proof Generation

Once the proof Setup step is done, you can create zk-SNARKs by providing a valid witness in the .json format specified in `abi` field in the response of the `POST /compile`.
You have two ways to get a proof from a valid witness: 

  - Calling the endpoint `POST /<PROGRAM_HASH>/compute-generate-proof`. This is the preferred and simplest option. This endpoint takes the proof arguments as inputs and delivers the final proof in the response under the `payload` field.
  - Calling the endpoints `POST /<PROGRAM_HASH>/compute-witness` and `POST /<PROGRAM_HASH>/generate-proof`. These two endpoints replicate the functionality of the ZoKrates CLI. `POST /<PROGRAM_HASH>/compute-witness` takes the proof arguments and generates the witness. The witness is the used by `<PROGRAM_HASH>/generate-proof` to generate the zk-SNARK.
  

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
