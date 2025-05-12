// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract MessageStorage {
    // Variable for storing message
    string private message;

    // Contract owner address
    address public owner;

    // An event that is emitted when a message is saved.
    event MessageStored(address indexed sender, string message);

    // Modifier to check that the function is called only by the owner
    modifier onlyOwner() {
        require(msg.sender == owner, "Only the owner can perform this operation.");
        _;
    }

    // Constructor that sets the contract's owner
    constructor() {
        owner = msg.sender;
    }

    /**
     * @dev Function to save a message
     * @param _message New message to store
     */
    function storeMessage(string memory _message) public onlyOwner {
        // Checking if a message is not empty
        require(bytes(_message).length > 0, "Message cannot be empty");

        // Saving a message
        message = _message;

        // Emit an event about saving message
        emit MessageStored(msg.sender, _message);
    }

    /**
     * @dev Function to get saved message
     * @return Saved message
     */
    function retrieveMessage() public view returns (string memory) {
        return message;
    }

    /**
     * @dev Function to change the owner of a contract
     * @param newOwner New owner's address
     */
    function transferOwnership(address newOwner) public onlyOwner {
        require(newOwner != address(0), "Invalid new owner address");
        owner = newOwner;
    }
}
