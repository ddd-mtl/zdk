#!/bin/bash

zits --default-zome-name zPathExplorer -d "import {ItemLink, SweepInterval, SweepResponse} from './deps.types';" -i crates/path_explorer_types -i crates/path_explorer -o webcomponents/src/bindings/path-explorer.ts

zits --default-zome-name zTasker -i playground/zomes/tasker -i playground/zomes/tasker_model -o playground/webapp/src/bindings/tasker.ts
