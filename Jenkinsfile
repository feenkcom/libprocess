import hudson.tasks.test.AbstractTestResultAction
import hudson.model.Actionable
import hudson.tasks.junit.CaseResult

pipeline {
    agent none
    parameters {
        choice(name: 'BUMP', choices: ['minor', 'patch', 'major'], description: 'What to bump when releasing') }
    options {
        buildDiscarder(logRotator(numToKeepStr: '50'))
        disableConcurrentBuilds()
    }
    environment {
        GITHUB_TOKEN = credentials('githubrelease')
        AWSIP = 'ec2-18-197-145-81.eu-central-1.compute.amazonaws.com'

        LIBRARY_NAME = 'libProcess'
        REPOSITORY_OWNER = 'feenkcom'
        REPOSITORY_NAME = 'libprocess'

        MACOS_INTEL_TARGET = 'x86_64-apple-darwin'
        MACOS_M1_TARGET = 'aarch64-apple-darwin'

        WINDOWS_SERVER_NAME = 'daffy-duck'
        WINDOWS_AMD64_TARGET = 'x86_64-pc-windows-msvc'

        LINUX_SERVER_NAME = 'mickey-mouse'
        LINUX_AMD64_TARGET = 'x86_64-unknown-linux-gnu'
    }

    stages {
        stage ('Read tool versions') {
            agent {
                label "${MACOS_M1_TARGET}"
            }
            steps {
                script {
                    FEENK_RELEASER_VERSION = sh (
                        script: "cat feenk-releaser.version",
                        returnStdout: true
                    ).trim()
                }
                echo "Will release using feenk-releaser ${FEENK_RELEASER_VERSION}"
            }
        }
        stage ('Parallel build') {
            parallel {
                stage ('MacOS x86_64') {
                    agent {
                        label "${MACOS_INTEL_TARGET}"
                    }
                    environment {
                        TARGET = "${MACOS_INTEL_TARGET}"
                        EXTENSION = "dylib"
                        PATH = "$HOME/.cargo/bin:/usr/local/bin/:$PATH"
                    }

                    steps {
                        sh 'git clean -fdx'
                        sh "cargo build --bin ${LIBRARY_NAME} --release"

                        sh "mv target/release/${LIBRARY_NAME}.${EXTENSION} ${LIBRARY_NAME}-${TARGET}.${EXTENSION}"

                        stash includes: "${LIBRARY_NAME}-${TARGET}.${EXTENSION}", name: "${TARGET}"
                    }
                }
                stage ('MacOS M1') {
                    agent {
                        label "${MACOS_M1_TARGET}"
                    }

                    environment {
                        TARGET = "${MACOS_M1_TARGET}"
                        EXTENSION = "dylib"
                        PATH = "$HOME/.cargo/bin:/opt/homebrew/bin:$PATH"
                    }

                    steps {
                        sh 'git clean -fdx'
                        sh "cargo build --bin ${LIBRARY_NAME} --release"

                        sh "mv target/release/${LIBRARY_NAME}.${EXTENSION} ${LIBRARY_NAME}-${TARGET}.${EXTENSION}"

                        stash includes: "${LIBRARY_NAME}-${TARGET}.${EXTENSION}", name: "${TARGET}"
                    }
                }

                stage ('Linux x86_64') {
                    agent {
                        label "${LINUX_AMD64_TARGET}-${LINUX_SERVER_NAME}"
                    }
                    environment {
                        TARGET = "${LINUX_AMD64_TARGET}"
                        EXTENSION = "so"
                        PATH = "$HOME/.cargo/bin:$PATH"
                    }

                    steps {
                        sh 'git clean -fdx'
                        sh "cargo build --bin ${LIBRARY_NAME} --release"

                        sh "mv target/release/${LIBRARY_NAME}.${EXTENSION} ${LIBRARY_NAME}-${TARGET}.${EXTENSION}"

                        stash includes: "${LIBRARY_NAME}-${TARGET}.${EXTENSION}", name: "${TARGET}"
                    }
                }

                stage ('Windows x86_64') {
                    agent {
                        label "${WINDOWS_AMD64_TARGET}-${WINDOWS_SERVER_NAME}"
                    }

                    environment {
                        TARGET = "${WINDOWS_AMD64_TARGET}"
                        EXTENSION = "so"
                        LLVM_HOME = 'C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\BuildTools\\VC\\Tools\\Llvm\\x64'
                        LIBCLANG_PATH = "${LLVM_HOME}\\bin"
                        CMAKE_PATH = 'C:\\Program Files\\CMake\\bin'
                        MSBUILD_PATH = 'C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\BuildTools\\MSBuild\\Current\\Bin'
                        CARGO_HOME = "C:\\.cargo"
                        CARGO_PATH = "${CARGO_HOME}\\bin"
                        PATH = "${CARGO_PATH};${LIBCLANG_PATH};${MSBUILD_PATH};${CMAKE_PATH};$PATH"
                    }

                    steps {
                        powershell 'git clean -fdx'

                        powershell "cargo build --bin ${LIBRARY_NAME} --release"
                        powershell "Move-Item -Path target/release/${LIBRARY_NAME}.${EXTENSION} -Destination ${LIBRARY_NAME}-${TARGET}.${EXTENSION}"
                        stash includes: "${LIBRARY_NAME}-${TARGET}.${EXTENSION}", name: "${TARGET}"
                    }
                }
            }
        }
        stage ('Sign and Notarize Mac') {
            agent {
                label "${MACOS_M1_TARGET}"
            }

            environment {
                TARGET = "${MACOS_M1_TARGET}"
                PATH = "$HOME/.cargo/bin:/opt/homebrew/bin:$PATH"
                CERT = credentials('devcertificate')
                APPLEPASSWORD = credentials('notarizepassword')
            }

            steps {
                sh 'git clean -fdx'
                unstash "${MACOS_INTEL_TARGET}"
                unstash "${MACOS_M1_TARGET}"
                sh "curl -o feenk-signer -LsS  https://github.com/feenkcom/feenk-signer/releases/download/${FEENK_SIGNER_VERSION}/feenk-signer-${TARGET}"
                sh "chmod +x feenk-signer"

                sh "./feenk-signer ${LIBRARY_NAME}-${MACOS_INTEL_TARGET}"
                sh "./feenk-signer ${LIBRARY_NAME}-${MACOS_M1_TARGET}"

                stash includes: "${LIBRARY_NAME}-${MACOS_INTEL_TARGET}", name: "${MACOS_INTEL_TARGET}"
                stash includes: "${LIBRARY_NAME}-${MACOS_M1_TARGET}", name: "${MACOS_M1_TARGET}"
            }
        }
        stage ('Deployment') {
            agent {
                label "${MACOS_M1_TARGET}"
            }
            environment {
                TARGET = "${MACOS_M1_TARGET}"
            }
            when {
                expression {
                    (currentBuild.result == null || currentBuild.result == 'SUCCESS') && env.BRANCH_NAME.toString().equals('main')
                }
            }
            steps {
                unstash "${LINUX_AMD64_TARGET}"
                unstash "${MACOS_INTEL_TARGET}"
                unstash "${MACOS_M1_TARGET}"
                unstash "${WINDOWS_AMD64_TARGET}"

                sh "curl -o feenk-releaser -LsS https://github.com/feenkcom/releaser-rs/releases/download/${FEENK_RELEASER_VERSION}/feenk-releaser-${TARGET}"
                sh "chmod +x feenk-releaser"

                sh """
                ./feenk-releaser \
                    --owner ${REPOSITORY_OWNER} \
                    --repo ${REPOSITORY_NAME} \
                    --token GITHUB_TOKEN \
                    --bump ${params.BUMP} \
                    --auto-accept \
                    --assets \
                        ${LIBRARY_NAME}-${LINUX_AMD64_TARGET}.so \
                        ${LIBRARY_NAME}-${MACOS_INTEL_TARGET}.dylib \
                        ${LIBRARY_NAME}-${MACOS_M1_TARGET}.dylib \
                        ${LIBRARY_NAME}-${WINDOWS_AMD64_TARGET}.dll """
            }
        }
    }
}
