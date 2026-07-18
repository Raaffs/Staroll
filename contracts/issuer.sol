// SPDX-License-Identifier: MPL-2.0
pragma solidity ^0.8.26;

contract IssuerRegistry {
    struct Issuer {
        string name;
        bool isApproved;
        bool isActive;
    }

    address public owner;
    mapping(address => Issuer) public issuers;
    address[] public issuerAddresses;

    modifier onlyOwner() {
        require(msg.sender == owner, "Not the owner");
        _;
    }

    constructor() {
        owner = msg.sender;
    }

    function registerIssuer(address _issuer, string calldata _name) external {
        require(_issuer != address(0), "Invalid address");
        // Simple mapping check: if isActive is true, they already exist
        require(!issuers[_issuer].isActive, "Issuer already registered");

        issuers[_issuer] = Issuer({
            name: _name,
            isApproved: false,
            isActive: true
        });

        issuerAddresses.push(_issuer);
    }

    function approveIssuer(address _issuer) external onlyOwner {
        require(issuers[_issuer].isActive, "Issuer does not exist");
        
        issuers[_issuer].isApproved = true;
    }

    function getAllIssuerAddresses(uint256 offset, uint256 limit) external view returns (address[] memory) {
        uint256 totalAddresses = issuerAddresses.length;
        
        if (offset >= totalAddresses || limit == 0) {
            return new address[](0);
        }

        uint256 size = limit;
        if (offset + limit > totalAddresses) {
            size = totalAddresses - offset;
        }

        address[] memory page = new address[](size);
        for (uint256 i = 0; i < size; i++) {
            page[i] = issuerAddresses[offset + i];
        }

        return page;
    }
}