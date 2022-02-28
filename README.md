# gun-gui

First, clone the lastest version of gun into `src-tauri/src/`:
```
git clone https://github.com/GoUpNumber/gun -o src-tauri/src
```

Replace the `Cargo.toml` with the one that includes tauri:
```
mv Cargo.toml src-tauri/src/
```

`src-tauri/src/main.rs` contains the bindings for Tauri, which are called from the components in `src/components/`.


## Project setup
```
yarn install
```

### Compiles and hot-reloads for development
```
yarn serve
```

### Compiles and minifies for production
```
yarn build
```

### Lints and fixes files
```
yarn lint
```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).
