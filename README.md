# mobile-waranty
# Warranty Contract

## Project Vision

The Warranty Contract is designed to provide a blockchain-based solution for managing product warranties. The vision behind this project is to create a transparent and tamper-proof system where users can register warranties for their products and easily verify the validity of those warranties.

### Features

- **Registration**: Users can register warranties for their products by providing the product's IMEI (International Mobile Equipment Identity) number, purchase date, and warranty period.
  
- **Validation**: The system allows users to verify the validity of warranties by checking the expiration date based on the purchase date and warranty period.
  
- **Deactivation**: Admin users can deactivate warranties if needed, making them inactive and no longer valid.
  
- **Extension**: Admin users can extend the warranty period for a product, provided the warranty is still active.

## Usage

### Initialization

Before using the contract functionalities, the contract needs to be initialized by an admin user. This is done by calling the `initialize` function and passing the admin's address.

### Registering Warranties

Users can register warranties for their products using the `register_warranty` function. This requires providing the product's IMEI, purchase date (in Unix timestamp format), and warranty period (in seconds).

### Checking Warranties

To check the validity of a warranty, users can call the `check_warranty` function and provide the IMEI of the product. This will return a boolean value indicating whether the warranty is still active.

### Deactivating Warranties

Admin users have the authority to deactivate warranties using the `deactivate_warranty` function. This will mark the warranty as inactive and invalid.

### Extending Warranties

Admin users can extend the warranty period for a product by calling the `extend_warranty` function and providing the IMEI of the product along with the additional period to be added.
