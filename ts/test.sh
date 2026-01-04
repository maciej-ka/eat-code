#!/bin/bash
[ ! -d ./node_modules ] && npm i
npm test
