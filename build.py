import os
import glob
import shutil
import subprocess

BUILD_CONFIG='RelWithDebInfo'

cmake_args = ['-DCMAKE_BUILD_TYPE=' + BUILD_CONFIG, '../']

if __name__ == '__main__':
  if not os.path.exists('build'):
    os.mkdir('build')
  os.chdir('build')
  subprocess.run(['cmake', *cmake_args])
  subprocess.run(['cmake', '--build', '.', '--config', BUILD_CONFIG])
  os.chdir('..')