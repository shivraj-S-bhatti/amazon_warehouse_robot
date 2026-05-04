from setuptools import find_packages, setup
import os
from glob import glob

package_name = "robomaster_detection"

setup(
    name=package_name,
    version="0.1.0",
    packages=find_packages(exclude=["test"]),
    data_files=[
        # ament index
        (
            "share/ament_index/resource_index/packages",
            [f"resource/{package_name}"],
        ),
        # package.xml
        (f"share/{package_name}", ["package.xml"]),
        # launch files
        (
            os.path.join("share", package_name, "launch"),
            glob("launch/*.launch.py"),
        ),
        # config files (if any)
        (
            os.path.join("share", package_name, "config"),
            glob("config/*.yaml"),
        ),
    ],
    install_requires=["setuptools"],
    zip_safe=True,
    maintainer="Your Name",
    maintainer_email="you@example.com",
    description="Person and marker detection for the RoboMaster robot in ROS 2.",
    license="Apache-2.0",
    entry_points={
        "console_scripts": [
            "detection_node  = robomaster_detection.detection_node:main",
            "avoidance_node  = robomaster_detection.avoidance_node:main"
        ],
    },
)
