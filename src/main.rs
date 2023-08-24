use std::fs;
use std::io::Write;
use std::path::Path;
use xshell::{cmd, Shell};

fn check_and_install(sh: &Shell, package: &str, install_cmd: &str, is_npm: bool) -> bool {
    if is_npm {
        let output = cmd!(sh, "npm list -g --depth=0").read().unwrap();
        if output.contains(package) {
            println!("Package {} exists, skipping installation.", package);
            false
        } else {
            cmd!(sh, "{install_cmd}").run().unwrap();
            true
        }
    } else {
        if Path::new(package).exists() {
            println!("File {} exists, skipping installation.", package);
            false
        } else {
            cmd!(sh, "{install_cmd}").run().unwrap();
            true
        }
    }
}

// fn check_and_install(sh: &Shell, package: &str, install_cmd: &str, is_npm: bool) -> bool {
//     if is_npm {
//         let output = cmd!(sh, "npm list -g --depth=0").read().unwrap();
//         if output.contains(package) {
//             println!("Package {} exists, skipping installation.", package);
//             false
//         } else {
//             let _output = read(install_cmd).unwrap();
//             true
//         }
//     } else {
//         if Path::new(package).exists() {
//             println!("File {} exists, skipping installation.", package);
//             false
//         } else {
//             let _output = read(install_cmd).unwrap();
//             true
//         }
//     }
// }

fn main() {
    let sh = Shell::new().unwrap();
    // // Install necessary packages
    // cmd!(sh, "sudo apt install -y git curl make jq")
    //     .run()
    //     .unwrap();
    // cmd!(sh, "sudo apt update").run().unwrap();

    // // Install Go
    // let go_installed = check_and_install(
    //     &sh,
    //     "/usr/bin/go",
    //     "wget https://go.dev/dl/go1.20.linux-amd64.tar.gz",
    //     false,
    // );
    // if go_installed {
    //     fs::write("/home/username/.bashrc", "export GOROOT=/usr/lib/go").unwrap();
    // }

    // Install Foundry
    check_and_install(
        &sh,
        "/home/ubuntu/.foundry/",
        "curl -L https://foundry.paradigm.xyz | bash",
        false,
    );

    // let nvm_installed = check_and_install(
    //     &sh,
    //     "$HOME/.nvm",
    //     "bash -c 'curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.38.0/install.sh | bash'",
    //     false,
    // );
    // if nvm_installed {
    //     cmd!(
    //         sh,
    //         "source $HOME/.nvm/nvm.sh && nvm install 18 && nvm use 18"
    //     )
    //     .run()
    //     .unwrap();
    // } else {
    //     cmd!(
    //         sh,
    //         "source $HOME/.nvm/nvm.sh && nvm install 18 && nvm use 18"
    //     )
    //     .run()
    //     .unwrap();
    // }

    let nvm_installed = check_and_install(
        &sh,
        "/home/ubuntu/.nvm/",
        "curl https://raw.githubusercontent.com/creationix/nvm/master/install.sh | bash ",
        false,
    );
    if nvm_installed {
        println!(
            "{:?}",
            cmd!(
                sh,
                "source /home/ubuntu/.nvm/nvm.sh && nvm install 18 && nvm use 18"
            )
            .run()
            .unwrap()
        );
    } else {
        println!(
            "{:?}",
            cmd!(
                sh,
                "source /home/ubuntu/.nvm/nvm.sh && nvm install 18 && nvm use 18"
            )
            .run()
            .unwrap()
        );
    }

    // Install pnpm
    check_and_install(&sh, "pnpm", "npm install -g pnpm", true);

    // Clone Ethereum Optimism repositories and build
    cmd!(
        sh,
        "cd ~ && git clone https://github.com/ethereum-optimism/optimism.git"
    )
    .run()
    .unwrap();
    cmd!(sh, "cd ~/optimism && make op-node op-batcher op-proposer")
        .run()
        .unwrap();
    cmd!(sh, "cd ~/optimism && pnpm build").run().unwrap();

    cmd!(
        sh,
        "cd ~ && git clone https://github.com/ethereum-optimism/op-geth.git"
    )
    .run()
    .unwrap();
    cmd!(sh, "cd ~/op-geth && make geth").run().unwrap();

    // Generate wallets
    let wallets = ["Admin", "Proposer", "Batcher", "Sequencer"];

    for wallet in wallets.iter() {
        let output = cmd!(
            sh,
            "cd ~/optimism/packages/contracts-bedrock && echo {wallet}"
        )
        .read()
        .unwrap();
        println!("{}", output);

        fs::write("output.txt", format!("{}\n", output)).unwrap();

        let output = cmd!(
            sh,
            "cd ~/optimism/packages/contracts-bedrock && cast wallet new"
        )
        .read()
        .unwrap();
        println!("{}", output);

        fs::OpenOptions::new()
            .append(true)
            .open("output.txt")
            .unwrap()
            .write_all(format!("{}\n", output).as_bytes())
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xshell::Shell;

    #[test]
    fn test_check_and_install_npm_package() -> Result<(), Box<dyn std::error::Error>> {
        let sh = Shell::new().unwrap();
        let package = "npm"; // replace with a global npm package installed on your system
        let install_cmd = "npm install -g npm"; // replace with the install command for the package
        let is_npm = true;

        let result = check_and_install(&sh, package, install_cmd, is_npm);
        assert_eq!(result, false); // should return false as the package is already installed
        Ok(())
    }

    #[test]
    fn test_check_and_install_file() -> Result<(), Box<dyn std::error::Error>> {
        let sh = Shell::new().unwrap();
        let package = "test_file"; // replace with a file that exists in your root directory
        let install_cmd = "touch test_file"; // replace with the command to create the file
        let is_npm = false;

        let result = check_and_install(&sh, package, install_cmd, is_npm);
        assert_eq!(result, false); // should return false as the file already exists
        Ok(())
    }
}
