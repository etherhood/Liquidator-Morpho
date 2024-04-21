// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity 0.8.25;

interface IOracle {
    function price() external view returns (uint256);
}
