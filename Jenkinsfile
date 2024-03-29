import hudson.tasks.test.AbstractTestResultAction
import hudson.model.Actionable
import hudson.tasks.junit.CaseResult

pipeline {
    agent none
    parameters {
        booleanParam(name: 'BUILD', defaultValue: false, description: 'Set to true to build a new version')
        choice(name: 'BUMP', choices: ['minor', 'patch', 'major'], description: 'What to bump when releasing') }
    options {
        buildDiscarder(logRotator(numToKeepStr: '50'))
        disableConcurrentBuilds()
    }
    environment {
        GITHUB_TOKEN = credentials('githubrelease')

        LIBRARY_NAME = 'crypto'
        ANOTHER_LIBRARY_NAME = 'ssl'
        REPOSITORY_OWNER = 'feenkcom'
        REPOSITORY_NAME = 'libopenssl'

        MACOS_INTEL_TARGET = 'x86_64-apple-darwin'
        MACOS_M1_TARGET = 'aarch64-apple-darwin'

        WINDOWS_AMD64_SERVER_NAME = 'daffy-duck'
        WINDOWS_AMD64_TARGET = 'x86_64-pc-windows-msvc'
        WINDOWS_ARM64_SERVER_NAME = 'bugs-bunny'
        WINDOWS_ARM64_TARGET = 'aarch64-pc-windows-msvc'

        LINUX_AMD64_SERVER_NAME = 'mickey-mouse'
        LINUX_AMD64_TARGET = 'x86_64-unknown-linux-gnu'
        LINUX_ARM64_SERVER_NAME = 'peter-pan'
        LINUX_ARM64_TARGET = 'aarch64-unknown-linux-gnu'

        ANDROID_ARM64_TARGET = 'aarch64-linux-android'
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
            when {
                expression {
                    (currentBuild.result == null || currentBuild.result == 'SUCCESS') && env.BRANCH_NAME.toString().equals('main') && params.BUILD
                }
            }
            parallel {
                stage ('MacOS x86_64') {
                    agent {
                        label "${MACOS_INTEL_TARGET}"
                    }
                    environment {
                        TARGET = "${MACOS_INTEL_TARGET}"
                        EXTENSION = "dylib"
                    }

                    steps {
                        sh 'git clean -fdx'
                        sh "cargo run --package ${REPOSITORY_NAME}-builder --bin builder --release"

                        sh "mv target/${TARGET}/release/lib${LIBRARY_NAME}.${EXTENSION} lib${LIBRARY_NAME}-${TARGET}.${EXTENSION}"
                        sh "mv target/${TARGET}/release/lib${ANOTHER_LIBRARY_NAME}.${EXTENSION} lib${ANOTHER_LIBRARY_NAME}-${TARGET}.${EXTENSION}"

                        stash includes: "lib${LIBRARY_NAME}-${TARGET}.${EXTENSION}, lib${ANOTHER_LIBRARY_NAME}-${TARGET}.${EXTENSION}", name: "${TARGET}"
                    }
                }
                stage ('MacOS M1') {
                    agent {
                        label "${MACOS_M1_TARGET}"
                    }

                    environment {
                        TARGET = "${MACOS_M1_TARGET}"
                        EXTENSION = "dylib"
                    }

                    steps {
                        sh 'git clean -fdx'
                        sh "cargo run --package ${REPOSITORY_NAME}-builder --bin builder --release"

                        sh "mv target/${TARGET}/release/lib${LIBRARY_NAME}.${EXTENSION} lib${LIBRARY_NAME}-${TARGET}.${EXTENSION}"
                        sh "mv target/${TARGET}/release/lib${ANOTHER_LIBRARY_NAME}.${EXTENSION} lib${ANOTHER_LIBRARY_NAME}-${TARGET}.${EXTENSION}"

                        stash includes: "lib${LIBRARY_NAME}-${TARGET}.${EXTENSION}, lib${ANOTHER_LIBRARY_NAME}-${TARGET}.${EXTENSION}", name: "${TARGET}"
                    }
                }

                stage ('Linux x86_64') {
                    agent {
                        label "${LINUX_AMD64_TARGET}-${LINUX_AMD64_SERVER_NAME}"
                    }
                    environment {
                        TARGET = "${LINUX_AMD64_TARGET}"
                        EXTENSION = "so"
                        PATH = "$HOME/.cargo/bin:$PATH"
                    }

                    steps {
                        sh 'git clean -fdx'
                        sh "cargo run --package ${REPOSITORY_NAME}-builder --bin builder --release"

                        sh "mv target/${TARGET}/release/lib${LIBRARY_NAME}.${EXTENSION} lib${LIBRARY_NAME}-${TARGET}.${EXTENSION}"
                        sh "mv target/${TARGET}/release/lib${ANOTHER_LIBRARY_NAME}.${EXTENSION} lib${ANOTHER_LIBRARY_NAME}-${TARGET}.${EXTENSION}"

                        stash includes: "lib${LIBRARY_NAME}-${TARGET}.${EXTENSION}, lib${ANOTHER_LIBRARY_NAME}-${TARGET}.${EXTENSION}", name: "${TARGET}"
                    }
                }
                stage ('Linux arm64') {
                    agent {
                        label "${LINUX_ARM64_TARGET}-${LINUX_ARM64_SERVER_NAME}"
                    }
                    environment {
                        TARGET = "${LINUX_ARM64_TARGET}"
                        EXTENSION = "so"
                        PATH = "$HOME/.cargo/bin:$PATH"
                    }

                    steps {
                        sh 'git clean -fdx'
                        sh "cargo run --package ${REPOSITORY_NAME}-builder --bin builder --release"

                        sh "mv target/${TARGET}/release/lib${LIBRARY_NAME}.${EXTENSION} lib${LIBRARY_NAME}-${TARGET}.${EXTENSION}"
                        sh "mv target/${TARGET}/release/lib${ANOTHER_LIBRARY_NAME}.${EXTENSION} lib${ANOTHER_LIBRARY_NAME}-${TARGET}.${EXTENSION}"

                        stash includes: "lib${LIBRARY_NAME}-${TARGET}.${EXTENSION}, lib${ANOTHER_LIBRARY_NAME}-${TARGET}.${EXTENSION}", name: "${TARGET}"
                    }
                }

                stage ('Android arm64') {
                    agent {
                        label "${MACOS_M1_TARGET}"
                    }
                    environment {
                        TARGET = "${ANDROID_ARM64_TARGET}"
                        EXTENSION = "so"
                    }

                    steps {
                        sh 'git clean -fdx'
                        sh "cargo run --package ${REPOSITORY_NAME}-builder --bin builder --release -- --target ${TARGET}"

                        sh "mv target/${TARGET}/release/lib${LIBRARY_NAME}.${EXTENSION} lib${LIBRARY_NAME}-${TARGET}.${EXTENSION}"
                        sh "mv target/${TARGET}/release/lib${ANOTHER_LIBRARY_NAME}.${EXTENSION} lib${ANOTHER_LIBRARY_NAME}-${TARGET}.${EXTENSION}"

                        stash includes: "lib${LIBRARY_NAME}-${TARGET}.${EXTENSION}, lib${ANOTHER_LIBRARY_NAME}-${TARGET}.${EXTENSION}", name: "${TARGET}"
                    }
                }

                stage ('Windows x86_64') {
                    agent {
                        label "${WINDOWS_AMD64_TARGET}-${WINDOWS_AMD64_SERVER_NAME}"
                    }

                    environment {
                        TARGET = "${WINDOWS_AMD64_TARGET}"
                        EXTENSION = "dll"
                        LLVM_HOME = 'C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\BuildTools\\VC\\Tools\\Llvm\\x64'
                        LIBCLANG_PATH = "${LLVM_HOME}\\bin"
                        MSBUILD_PATH = 'C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\BuildTools\\MSBuild\\Current\\Bin'
                        MSVC_PATH = 'C:\\Program Files (x86)\\Microsoft Visual Studio\\2019\\BuildTools\\VC\\Tools\\MSVC\\14.29.30037\\bin\\Hostx64\\x64'
                        CARGO_HOME = "C:\\.cargo"
                        CARGO_PATH = "${CARGO_HOME}\\bin"
                        PERL_PATH = 'C:\\Strawberry\\perl\\site\\bin;C:\\Strawberry\\perl\\bin'
                        NASM_PATH  = 'C:\\Program Files\\NASM'
                        PATH = "${CARGO_PATH};${LIBCLANG_PATH};${MSBUILD_PATH};${CMAKE_PATH};${MSVC_PATH};${PERL_PATH};${NASM_PATH};$PATH"
                    }

                    steps {
                        powershell 'Remove-Item -Force -Recurse -Path target -ErrorAction Ignore'
                        powershell 'git clean -fdx'

                        powershell "cargo run --package ${REPOSITORY_NAME}-builder --bin builder --release -- --target ${TARGET}"

                        powershell "Move-Item -Path target/${TARGET}/release/${LIBRARY_NAME}.${EXTENSION} -Destination ${LIBRARY_NAME}-${TARGET}.${EXTENSION}"
                        powershell "Move-Item -Path target/${TARGET}/release/${ANOTHER_LIBRARY_NAME}.${EXTENSION} -Destination ${ANOTHER_LIBRARY_NAME}-${TARGET}.${EXTENSION}"

                        stash includes: "${LIBRARY_NAME}-${TARGET}.${EXTENSION}, ${ANOTHER_LIBRARY_NAME}-${TARGET}.${EXTENSION}", name: "${TARGET}"
                    }
                }
                stage ('Windows arm64') {
                    agent {
                        label "${WINDOWS_ARM64_TARGET}-${WINDOWS_ARM64_SERVER_NAME}"
                    }

                    environment {
                        TARGET = "${WINDOWS_ARM64_TARGET}"
                        EXTENSION = "dll"
                        CARGO_HOME = "C:\\.cargo"
                        CARGO_PATH = "${CARGO_HOME}\\bin"
                        NASM_PATH  = 'C:\\Program Files\\NASM'
                        PATH = "${CARGO_PATH};${NASM_PATH};$PATH"
                    }

                    steps {
                        powershell 'Remove-Item -Force -Recurse -Path target -ErrorAction Ignore'
                        powershell 'git clean -fdx'

                        powershell "cargo run --package ${REPOSITORY_NAME}-builder --bin builder --release -- --target ${TARGET}"

                        powershell "Move-Item -Path target/${TARGET}/release/${LIBRARY_NAME}.${EXTENSION} -Destination ${LIBRARY_NAME}-${TARGET}.${EXTENSION}"
                        powershell "Move-Item -Path target/${TARGET}/release/${ANOTHER_LIBRARY_NAME}.${EXTENSION} -Destination ${ANOTHER_LIBRARY_NAME}-${TARGET}.${EXTENSION}"

                        stash includes: "${LIBRARY_NAME}-${TARGET}.${EXTENSION}, ${ANOTHER_LIBRARY_NAME}-${TARGET}.${EXTENSION}", name: "${TARGET}"
                    }
                }
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
                    (currentBuild.result == null || currentBuild.result == 'SUCCESS') && env.BRANCH_NAME.toString().equals('main') && params.BUILD
                }
            }
            steps {
                unstash "${LINUX_AMD64_TARGET}"
                unstash "${LINUX_ARM64_TARGET}"
                unstash "${MACOS_INTEL_TARGET}"
                unstash "${MACOS_M1_TARGET}"
                unstash "${ANDROID_ARM64_TARGET}"
                unstash "${WINDOWS_AMD64_TARGET}"
                unstash "${WINDOWS_ARM64_TARGET}"

                sh "curl -o feenk-releaser -LsS https://github.com/feenkcom/releaser-rs/releases/download/${FEENK_RELEASER_VERSION}/feenk-releaser-${TARGET}"
                sh "chmod +x feenk-releaser"

                sh """
                ./feenk-releaser \
                    --owner ${REPOSITORY_OWNER} \
                    --repo ${REPOSITORY_NAME} \
                    --token GITHUB_TOKEN \
                    release \
                    --bump ${params.BUMP} \
                    --auto-accept \
                    --assets \
                        lib${LIBRARY_NAME}-${LINUX_AMD64_TARGET}.so \
                        lib${ANOTHER_LIBRARY_NAME}-${LINUX_AMD64_TARGET}.so \
                        lib${LIBRARY_NAME}-${LINUX_ARM64_TARGET}.so \
                        lib${ANOTHER_LIBRARY_NAME}-${LINUX_ARM64_TARGET}.so \
                        lib${LIBRARY_NAME}-${MACOS_INTEL_TARGET}.dylib \
                        lib${ANOTHER_LIBRARY_NAME}-${MACOS_INTEL_TARGET}.dylib \
                        lib${LIBRARY_NAME}-${MACOS_M1_TARGET}.dylib \
                        lib${ANOTHER_LIBRARY_NAME}-${MACOS_M1_TARGET}.dylib \
                        lib${LIBRARY_NAME}-${ANDROID_ARM64_TARGET}.so \
                        lib${ANOTHER_LIBRARY_NAME}-${ANDROID_ARM64_TARGET}.so \
                        ${LIBRARY_NAME}-${WINDOWS_AMD64_TARGET}.dll \
                        ${ANOTHER_LIBRARY_NAME}-${WINDOWS_AMD64_TARGET}.dll \
                        ${LIBRARY_NAME}-${WINDOWS_ARM64_TARGET}.dll \
                        ${ANOTHER_LIBRARY_NAME}-${WINDOWS_ARM64_TARGET}.dll """
            }
        }
    }
}
