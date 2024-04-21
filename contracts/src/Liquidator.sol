// SPDX-License-Identifier: GPL-2.0-or-later
pragma solidity 0.8.25;

import {IMorphoLiquidateCallback} from "./MorphoCallbacks.sol";
import {IMorpho, MarketParams} from "./IMorpho.sol";
import {IERC20} from "openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";
import {SafeERC20} from "openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";
import {Address} from "openzeppelin-contracts/contracts/utils/Address.sol";

/// @author etherhood
/// @notice Liquidator contract which sells collateral for debt tokens
/// @notice Tokens should never be left over in this contract
contract Liquidator is IMorphoLiquidateCallback {
    using SafeERC20 for IERC20;
    using Address for address;

    struct LiquidateParams {
        address swapper;
        address collateral;
        address debt;
        bytes swapData;
    }

    IMorpho internal constant morpho = IMorpho(0xBBBBBbbBBb9cC5e90e3b3Af64bdAF62C37EEFFCb);

    function liquidateUser(
        MarketParams memory marketParams,
        address user,
        address swapper,
        uint256 seizedAssets,
        bytes calldata swapData
    ) external {
        morpho.liquidate(
            marketParams,
            user,
            seizedAssets,
            0,
            abi.encode(LiquidateParams(swapper, marketParams.collateralToken, marketParams.loanToken, swapData))
        );

        IERC20 collateral = IERC20(marketParams.collateralToken);

        uint256 balance = collateral.balanceOf(address(this));

        if (balance > 0) {
            collateral.safeTransfer(msg.sender, balance);
        }

        IERC20 debt = IERC20(marketParams.loanToken);

        balance = debt.balanceOf(address(this));

        if (balance > 0) {
            debt.safeTransfer(msg.sender, balance);
        }
    }

    function onMorphoLiquidate(uint256 repaidAssets, bytes calldata data) external {
        require(address(morpho) == msg.sender, "Liquidator: Invalid address");

        LiquidateParams memory liquidateParams = abi.decode(data, (LiquidateParams));

        uint256 collateralSeized = IERC20(liquidateParams.collateral).balanceOf(address(this));

        IERC20(liquidateParams.collateral).safeIncreaseAllowance(liquidateParams.swapper, collateralSeized);

        liquidateParams.swapper.functionCall(liquidateParams.swapData);

        IERC20(liquidateParams.debt).safeIncreaseAllowance(msg.sender, repaidAssets);
    }
}
