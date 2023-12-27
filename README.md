# tsm

tsm is a rust cli tool to manage tmux sessions. Switch to an existing session or create a new session.

### Features

- **Switch Session:** Switch to an existing tmux session.
- **Create Session:** Create a new tmux session with specified name.

### Requirements

- [tmux](https://github.com/tmux/tmux)

### Installation

1. Clone and build the project:
   ```sh
   git clone https://github.com/halshar/tsm.git
   cargo build --release
   ```
2. Install using cargo

   ```sh
   cargo install tsm
   ```

## Usage

1. Create a tmux binding in your `.tmux.conf` file
   ```bash
   # switch/create session
   # <prefix>+o will trigger the script
   bind-key o display-popup -E "tsm"
   ```
2. Within an active tmux session, run the command
   ```bash
   tsm
   ```

### Acknowledgments

- This project utilizes the [skim](https://github.com/lotabout/skim) rust library
- This project is inspired from [tmux-sessionizer](https://github.com/jrmoulton/tmux-sessionizer)

## Contributions

Contributions are welcome! Please open an issue or pull request for any changes or suggestions.

## License

This project is licensed under the [GPLv3 License](./LICENSE).
