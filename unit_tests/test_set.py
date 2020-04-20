from pyrpds import pset, s


def test_set():
    set_0 = pset()
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


def test_pset_constuctor():
    container = pset()
    assert len(container) == 0

    container = pset((0, "1", 2))
    assert len(container) == 3

    container = pset([0, "1", 2])
    assert len(container) == 3

    container = pset(range(5))
    assert len(container) == 5

    container = pset(map(lambda x: x, range(10)))
    assert len(container) == 10


def test_s_constuctor():
    container = s()
    assert len(container) == 0

    container = s(0, "1", 2, 6, 10)
    assert len(container) == 5


def test_iter():
    container = pset()
    for element in range(100):
        container.insert(element)
    for index, element in enumerate(sorted(container)):
        assert index == element
