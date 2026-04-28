idf_component_register(SRCS "hid_host_example.c" "usb_otg_detailed_logging.c"
                       INCLUDE_DIRS "."
                       PRIV_REQUIRES usb driver
                       )
