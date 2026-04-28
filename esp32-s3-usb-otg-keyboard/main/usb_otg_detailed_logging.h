#pragma once

#include "esp_log.h"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Log detailed USB PHY configuration state
 */
void log_usb_phy_configuration_state(void);

/**
 * @brief Log USB OTG register states during initialization
 */
void log_usb_otg_register_states(void);

/**
 * @brief Log USB port power and connection status
 */
void log_usb_port_status(void);

/**
 * @brief Log USB interrupt status and configuration
 */
void log_usb_interrupt_status(void);

/**
 * @brief Log USB FIFO configuration
 */
void log_usb_fifo_configuration(void);

/**
 * @brief Log GPIO matrix configuration for USB signals
 */
void log_usb_gpio_matrix_configuration(void);

/**
 * @brief Log USB device enumeration state
 */
void log_usb_device_enumeration_state(void);

/**
 * @brief Log USB host controller configuration
 */
void log_usb_host_controller_configuration(void);

/**
 * @brief Log complete USB OTG initialization state
 */
void log_complete_usb_otg_state(void);

#ifdef __cplusplus
}
#endif
