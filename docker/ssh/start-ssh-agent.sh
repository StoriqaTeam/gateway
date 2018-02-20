#!/usr/bin/env bash

eval `ssh-agent -s`
chmod a-r libstqbackend
chmod u+r libstqbackend
ssh-add libstqbackend
