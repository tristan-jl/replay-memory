from replay_memory import ReplayMemory


def test_overwritten():
    rm = ReplayMemory(10)
    rm.push_items(range(11))

    assert rm[0] == 10


def test_capacity_unchanged():
    rm = ReplayMemory(10)
    rm.push_items(range(100))

    assert len(rm) == 10
