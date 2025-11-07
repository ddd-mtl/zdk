#!/bin/bash
# TOP LEVEL
rm .running
rm .hc_live*
rm Cargo.lock
rm -rf target
# PLAYGROUND WEB-APP
rm -rf playground/webapp/dist
rm -rf playground/webapp/out-tsc
rm playground/webapp/tsconfig.tsbuildinfo
rm playground/webapp/ui.zip
