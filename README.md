<div align="center">
  <img src="/.github/images/github-header-image.webp" alt="GitHub Header Image" width="auto" />
  
  <!-- MIT License -->
  <a href="https://github.com/hadronomy/advent-of-code/blob/main/LICENSE">
    <img
      alt="Content License"
      src="https://img.shields.io/github/license/hadronomy/advent-of-code?style=for-the-badge&logo=starship&color=ee999f&logoColor=D9E0EE&labelColor=302D41"
    />
  </a>

  <!-- GitHub Repo Stars -->
  <a href="https://github.com/hadronomy/advent-of-code/stargazers">
    <img
      alt="Stars"
      src="https://img.shields.io/github/stars/hadronomy/advent-of-code?style=for-the-badge&logo=starship&color=c69ff5&logoColor=D9E0EE&labelColor=302D41"
    />
  </a>
  <p></p>
  <span>
    My solutions to the Advent of Code challenges.
  </span>
  <p></p>
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#license">License</a>
  <hr />

</div>


</div>

## Requirements

- rust - [Installation](https://rustup.rs)
- mise - [Installation](https://mise.jdx.dev)

`mise` Is used to manage the project dependencies and environment.
To ensure that you have the needed tools installed, run the following command:

```bash
mise trust
mise install
```

## Usage

To run the solution for a specific day, use the following command:

```bash
just run <year> <day_number> <part> # e.g. mise run 2024 1 1
```

To run the tests for a specific day, use the following command:

```bash
just test <year> <day_number> # e.g. mise test 2024 1
```

To run the benchmark for a specific day, use the following command:

```bash
just bench <year> <day_number> # e.g. just bench 2024 1
```

To see the available commands, use the following command:

```bash
just
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) for details.
