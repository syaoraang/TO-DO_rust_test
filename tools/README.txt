# GLADE FILE FROM GT3 TO GTK4

## Building image
podman build -t gt4-parser -f Gtk4-parser.Dockerfile

## EXECUTION
Run on the path where the .glade is
podman run --rm -v ${pwd}:/mnt localhost/gt4-parser:latest sample.glade