use clap::ValueEnum;

use super::cli::{ Dependency, DependenciesMod };

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum ComprehensiveType {
    WebpackReactJs,
    WebpackReactTs,
    ViteReactJs,
    ViteReactTs,
}

impl ComprehensiveType {
    pub fn get_dependencies(&self) -> Vec<Dependency> {
        match self {
            ComprehensiveType::WebpackReactJs => vec![],
            ComprehensiveType::WebpackReactTs =>
                vec![
                    Dependency {
                        name: "@babel/preset-typescript",
                        version: "^7.24.1",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "@types/node",
                        version: "^20.12.12",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "@types/react",
                        version: "^18.3.2",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "@types/react-dom",
                        version: "^18.3.0",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "@types/webpack",
                        version: "^5.28.5",
                        mod_type: DependenciesMod::Dev,
                    }
                ],
            ComprehensiveType::ViteReactJs =>
                vec![Dependency {
                    name: "@vitejs/plugin-react",
                    version: "^4.3.1",
                    mod_type: DependenciesMod::Dev,
                }],
            ComprehensiveType::ViteReactTs =>
                vec![
                    Dependency {
                        name: "@types/react",
                        version: "^18.3.2",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "@types/react-dom",
                        version: "^18.3.0",
                        mod_type: DependenciesMod::Dev,
                    },
                    Dependency {
                        name: "@vitejs/plugin-react",
                        version: "^4.3.1",
                        mod_type: DependenciesMod::Dev,
                    }
                ],
        }
    }
}
