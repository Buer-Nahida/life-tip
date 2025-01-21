# Life Tip

每过30分钟都会提醒你的贴心小应用

## 使用

NixOS / Nix 用户可以直接使用 Nix flake：

```nix
{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    life-tip = {
      url = "github:Buer-Nahida/life-tip";
      inputs.nixpkgs.follows = "nixpkgs";
    }
    # ...
  };
  outputs = { nixpkgs, ... }@inputs: {
    nixosConfigurations."example" = nixpkgs.lib.nixosSystem rec {
      system = "x86_64-linux";
      specialArgs = { inherit inputs; };
      modules = [
        { pkgs, inputs, ... }: {
          environment.systemPackages = [
            inputs.life-tip.packages.${pkgs.system}.default
          ];
        }
      ];
      # ...
    };
  };
}
```

其他发行版用户请自行编译安装：

```bash
git clone --depth 1 https://github.com/Buer-Nahida/life-tip.git
cd life-tip
# 确保你已经安装了 Rust 工具链
cargo build --release
sudo mv target/release/life-tip /usr/bin/life-tip
sudo chmod +x /usr/bin/life-tip
sudo chown root:root /usr/bin/life-tip
```
