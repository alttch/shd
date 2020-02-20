__version__ = '0.0.7'

import setuptools

with open("README.md", "r") as fh:
    long_description = fh.read()

setuptools.setup(
    name="shd",
    version=__version__,
    author='Altertech',
    author_email="div@altertech.com",
    description="Show HDD/SSD list",
    long_description=long_description,
    long_description_content_type='text/markdown',
    url="https://github.com/alttch/shd",
    packages=setuptools.find_packages(),
    license='MIT',
    install_requires=['pysmart', 'rapidtables', 'neotermcolor'],
    scripts=['shd'],
    classifiers=(
        'Programming Language :: Python :: 3',
        'License :: OSI Approved :: MIT License',
        'Topic :: System :: Hardware',
        'Topic :: System :: Monitoring'
    ),
)
