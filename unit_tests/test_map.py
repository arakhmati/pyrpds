from pyrpds import Map


def test_map():
    map_0 = Map()
    assert len(map_0) == 0
    assert hash(map_0) == -4800647303603446203
    assert set(map_0.values()) == set()

    map_1 = map_0.insert(0, 0)
    assert 0 in map_1
    assert map_1.get(0) == 0
    assert map_1[0] == 0
    assert len(map_1) == 1
    assert hash(map_1) == 2885792717948792520
    assert set(map_1.values()) == {0}

    map_2 = map_1.insert("1", 1)
    assert 0 in map_2
    assert map_2.get(0) == 0
    assert map_2[0] == 0
    assert "1" in map_2
    assert map_2.get("1") == 1
    assert map_2["1"] == 1
    assert len(map_2) == 2
    assert set(map_2.keys()) == {0, "1"}
    assert set(map_2.values()) == {0, 1}
    assert set(map_2.items()) == {(0, 0), ("1", 1)}

    map_3 = map_2.insert(2, "2")
    assert 0 in map_3
    assert map_3.get(0) == 0
    assert "1" in map_3
    assert map_3.get("1") == 1
    assert 2 in map_3
    assert map_3.get(2) == "2"
    assert len(map_3) == 3
    assert set(map_3.values()) == {0, "2", 1}

    map_4 = map_3.remove(2)
    assert 0 in map_4
    assert map_4.get(0) == 0
    assert "1" in map_4
    assert map_4.get("1") == 1
    assert len(map_4) == 2
    assert set(map_4.values()) == {0, 1}

    assert map_0 != map_1
    assert map_1 != map_2
    assert map_2 != map_3
    assert map_3 != map_4
    assert map_2 == map_4
    assert hash(map_2) == hash(map_4)


def test_pmap_constuctor():
    container = pmap()
    assert len(container) == 0

    container = pmap({0: "0", "1": 1, "2": "2"})
    assert len(container) == 3


def test_iter():
    container = Map()
    for key in range(100):
        value = key
        container.insert(key, value)
    for index, key in enumerate(sorted(container)):
        assert index == key and index == container[key]
