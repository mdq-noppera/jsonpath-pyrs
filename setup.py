import sys
from setuptools import setup

try:
    from setuptools_rust import Binding, RustExtension
except ImportError:
    import subprocess
    errno = subprocess.call(
        [sys.executable, '-m', 'pip', 'install', 'setuptools-rust'])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import Binding, RustExtension

setup_requires = ['setuptools-rust>=0.9.2']
install_requires = []

setup(name='jsonpath_pyrs',
      version='0.0.2',
      url = "https://github.com/niap0r/jsonpath-pyrs",
      classifiers=[
          'License :: OSI Approved :: MIT License',
          'Development Status :: 3 - Alpha',
          'Intended Audience :: Developers',
          'Programming Language :: Python',
          'Programming Language :: Rust',
          'Operating System :: POSIX :: Linux',
      ],
      rust_extensions=[
          RustExtension('jsonpath_pyrs._jsonpath_pyrs', 'Cargo.toml', binding=Binding.PyO3)],
      packages=['jsonpath_pyrs'],
      zip_safe=False)