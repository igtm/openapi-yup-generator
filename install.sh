#!/bin/sh

set -e

if [ $# -eq 0 ]; then
    echo "ERROR: Need to specify the install repository"
    exit 1
fi

owner="igtm"
repo="openapi-yup-generator"
exe_name="openapi-yup-generator"
githubUrl="https://github.com"
version=""
executable_folder="/usr/local/bin" # Eventually, the executable file will be placed here

get_arch() {
    # darwin/amd64: Darwin axetroydeMacBook-Air.local 20.5.0 Darwin Kernel Version 20.5.0: Sat May  8 05:10:33 PDT 2021; root:xnu-7195.121.3~9/RELEASE_X86_64 x86_64
    # linux/amd64: Linux test-ubuntu1804 5.4.0-42-generic #46~18.04.1-Ubuntu SMP Fri Jul 10 07:21:24 UTC 2020 x86_64 x86_64 x86_64 GNU/Linux
    a=$(uname -m)
    case ${a} in
        "x86_64" | "amd64" )
            echo "x86_64"
        ;;
        # "i386" | "i486" | "i586")
        #     echo "386"
        # ;;
        "aarch64" | "arm64" | "arm")
            echo "aarch64"
        ;;
        *)
            echo ${NIL}
        ;;
    esac
}

get_os(){
    # darwin: Darwin
    a=$(uname -s | awk '{print tolower($0)}')
    case ${a} in
        "darwin")
            echo "apple-darwin"
        ;;
        "windows")
            echo "pc-windows-msvc"
        ;;
        "linux")
            echo "unknown-linux-gnu"
        ;;
        *)
            echo ${NIL}
        ;;
    esac
}

# parse flag
for i in "$@"; do
    case $i in
        -b=*)
            executable_folder="${i#*=}"
            shift # past argument=value
        ;;
        -v=*)
            version="${i#*=}"
            shift # past argument=value
        ;;
        *)
            # unknown option
        ;;
    esac
done

downloadFolder="/tmp"
mkdir -p ${downloadFolder} # make sure download folder exists
os=$(get_os)
arch=$(get_arch)

# if not specified
if [[ ! "$version" ]]; then
    # latest version: (eg: v1.0.0)
    version=$(
        command curl -sSf ${githubUrl}/${owner}/${repo}/releases |
        command grep -o -E "/${owner}/${repo}/tree/(v[0-9]+\.){1}[0-9]+(\.[0-9]+)?" |
        command grep -o -E "(v[0-9]+\.){1}[0-9]+(\.[0-9]+)?" |
        command head -n 1
    )
    if [[ ! "$version" ]]; then exit 1; fi
fi


file_name="${exe_name}_${version}_${arch}-${os}.tar.gz" # the file name should be download
downloaded_file="${downloadFolder}/${file_name}" # the file path should be download
asset_uri="${githubUrl}/${owner}/${repo}/releases/download/${version}/${file_name}"

echo "[1/3] Download ${asset_uri} to ${downloadFolder}"
rm -f ${downloaded_file}
curl --fail --location --output "${downloaded_file}" "${asset_uri}"

echo "[2/3] Install ${exe_name} to the ${executable_folder}"
tar -xz -f ${downloaded_file} -C ${executable_folder}
exe=${executable_folder}/${exe_name}
chmod +x ${exe}

echo "[3/3] Set environment variables"
echo "${exe_name} was installed successfully to ${exe}"

exit 0