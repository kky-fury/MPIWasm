#!/usr/bin/env python3

import json
import os
import platform
from pprint import pprint
import shutil
from subprocess import PIPE, Popen
import sys
from typing import Any, Dict, List

import requests

WASI_SDK_GITHUB_URL = "https://api.github.com/repos/WebAssembly/wasi-sdk"
SYSTEM_NAME_TO_ASSET_SUFFIX = {
    "Darwin": "macos.tar.gz",
    "Linux": "linux.tar.gz",
}


def get_releases() -> List[Dict[str, Any]]:
    r = requests.get(WASI_SDK_GITHUB_URL + "/releases", timeout=30)

    if r.status_code == 200:
        return r.json()
    elif r.status_code == 404:
        sys.exit("WASI SDK could not be found at " + WASI_SDK_GITHUB_URL + ".")
    else:
        sys.exit("Failed fetching releases for WASI SDK. Is github.com up?")



def print_available_tags(tags: List[str], default_tag: str):
    for t in tags:
        msg = t

        if t == default_tag:
            msg += " *"
    
        print(msg)


def query_tag_to_install(releases: List[Dict[str, Any]]) -> str:
    tags = [r["tag_name"] for r in releases]
    default_tag = next(r["tag_name"] for r in releases if not r["prerelease"])
    
    print("Available WASI SDK Releases:")
    print_available_tags(tags, default_tag)
    tag_to_install = input("Release to install [" + default_tag + "]: ")

    if tag_to_install == "":
        return default_tag
    else:
        if tag_to_install not in tags:
            print("Invalid Release: " + tag_to_install)
            return query_tag_to_install(releases)
        else:
            return tag_to_install


def remove_prefix(text: str, prefix: str) -> str:
    if text.startswith(prefix):
        return text[len(prefix):]
    else:
        raise ValueError("'" + prefix + "' is not a prefix of '" + text + "'")


def query_install_directory(tag_to_install: str) -> str:
    version = remove_prefix(tag_to_install, "wasi-sdk-")
    user_home = os.path.expanduser("~")
    default_directory = os.path.join(user_home, "opt", "wasi-sdk", version)
    
    install_directory = input("Install directory [" + default_directory + "]: ")
    
    if install_directory == "":
        return default_directory
    else:
        return install_directory



def download_to_install_directory(
    releases: List[Dict[str, Any]],
    system_name: str,
    tag_to_install: str,
    install_directory: str,
):
    release_to_install = next(r for r in releases if r["tag_name"] == tag_to_install)
    
    asset_suffix = SYSTEM_NAME_TO_ASSET_SUFFIX[system_name]
    asset_to_install = next(
        (
            a for a in release_to_install["assets"]
            if a["name"].startswith("wasi-sdk") and a["name"].endswith(asset_suffix)
        ),
        None,
    )

    if asset_to_install is None:
        sys.exit(
            "No suitable asset found for release '" + tag_to_install + "' and system '" + system_name + "'. "
            "Make sure that releases are still named 'wasi-sdk-{version}-" + asset_suffix + "'."
        )
    
    if not os.path.exists(install_directory):
        os.makedirs(install_directory)
    elif len(os.listdir(install_directory)) > 0:
        sys.exit("Install directory '" + install_directory + "' is not empty.")

    
    curl_process = Popen(["curl", "-L", asset_to_install["browser_download_url"]], stdout=PIPE)
    tar_process = Popen(["tar", "xzf", "-", "--strip", "1", "-C", install_directory], stdin=curl_process.stdout, stdout=PIPE)
    curl_process.stdout.close()  # Allow curl to receive a SIGPIPE if tar exits.
    output = tar_process.communicate()[0]

    # Add mpi.h header file to sysroot
    shutil.copy(
        os.path.join(".", "include", "mpi.h"),
        os.path.join(install_directory, "share", "wasi-sysroot", "include"),
    )


def main():
    if len(sys.argv) <= 1:
        tag_to_install = None
        install_directory = None
    elif len(sys.argv) == 2:
        tag_to_install = sys.argv[1]
        install_directory = None
    else:
        # len(sys.argv) >= 3
        tag_to_install = sys.argv[1]
        install_directory = sys.argv[2]

    system_name = platform.system()
    if system_name not in ["Darwin", "Linux"]:
        sys.exit("System '" + system_name + "' is not supported yet.")

    releases = get_releases()

    if tag_to_install is None:
        tag_to_install = query_tag_to_install(releases)
    if install_directory is None:
        install_directory = query_install_directory(tag_to_install)
    
    download_to_install_directory(
        releases=releases,
        system_name=system_name,
        tag_to_install=tag_to_install,
        install_directory=install_directory,
    )


if __name__ == "__main__":
    main()
