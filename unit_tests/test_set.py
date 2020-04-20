from pyrpds import Set


def test_set():
    set_0 = Set()
    assert len(set_0) == 0
    assert hash(set_0) == -4800647303603446203

    set_1 = set_0.insert(0)
    assert 0 in set_1
    assert len(set_1) == 1
    assert hash(set_1) == -8559946577813192710

    set_2 = set_1.insert("1")
    assert 0 in set_2
    assert "1" in set_2
    assert len(set_2) == 2

    set_3 = set_2.insert(2)
    assert 0 in set_3
    assert "1" in set_3
    assert 2 in set_3
    assert len(set_3) == 3

    set_4 = set_3.remove(2)
    assert 0 in set_2
    assert "1" in set_2
    assert len(set_4) == 2

    assert set_0 != set_1
    assert set_1 != set_2
    assert set_2 != set_3
    assert set_3 != set_4
    assert set_2 == set_4
    assert hash(set_2) == hash(set_4)


def test_iter():
    container = Set()
    for element in range(100):
        container.insert(element)
    for index, element in enumerate(sorted(container)):
        assert index == element
