import unittest
from numba import njit

from hftbacktest import (
    PyAssetBuilder,
    PyAssetType,
    PyExchangeKind,
    PyLatencyModel,
    build_backtester,
    MultiAssetMultiExchangeBacktest, ANY_ASSET
)


@njit
def test_run(hbt):
    order_id = 0
    while hbt.elapse(10_000_000_000) == 0:
        current_timestamp = hbt.current_timestamp()
        depth = hbt.depth_typed(0)
        best_bid = depth.best_bid()
        best_ask = depth.best_ask()

        # trades = hbt.trade_typed(0)
        #
        # i = 0
        # for trade in trades:
        #     print(trade.local_ts, trade.px, trade.qty)
        #     i += 1
        #     if i > 5:
        #         break

        hbt.clear_last_trades(ANY_ASSET)

        cnt = 0
        orders = hbt.orders(0)
        values = orders.values()
        while True:
            order = values.next()
            if order is None:
                break
            cnt += 1
            print(order.order_id, order.side, order.price_tick, order.qty)

        hbt.clear_inactive_orders(ANY_ASSET)

        if cnt <= 2:
            hbt.submit_buy_order(0, order_id, best_bid, 1, 1, 0, False)
            order_id += 1
            hbt.submit_sell_order(0, order_id, best_ask, 1, 1, 0, False)
            order_id += 1

        print(current_timestamp, best_bid, best_ask)


class TestFFI(unittest.TestCase):
    def setUp(self) -> None:
        pass

    def test_run_backtest(self):
        asset = PyAssetBuilder()
        asset.asset_type(PyAssetType.LinearAsset)
        asset.data(['tmp_20240501.npz'])
        asset.exchange(PyExchangeKind.NoPartialFillExchange)
        asset.latency_model(PyLatencyModel.ConstantLatency)
        asset.trade_len(1000)

        raw_hbt = build_backtester([asset])

        hbt = MultiAssetMultiExchangeBacktest(raw_hbt.as_ptr())
        test_run(hbt)
