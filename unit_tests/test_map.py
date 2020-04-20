from pyrpds import Map


def test_map():
    map_0 = Map()
    assert len(map_0) == 0
    assert hash(map_0) == -4800647303603446203

    map_1 = map_0.insert(0, 0)
    assert 0 in map_1
    assert map_1.get(0) == 0
    assert len(map_1) == 1
    assert hash(map_1) == 2885792717948792520

    map_2 = map_1.insert("1", 1)
    assert 0 in map_2
    assert map_2.get(0) == 0
    assert "1" in map_2
    assert map_2.get("1") == 1
    assert len(map_2) == 2

    map_3 = map_2.insert(2, "2")
    assert 0 in map_3
    assert map_3.get(0) == 0
    assert "1" in map_3
    assert map_3.get("1") == 1
    assert 2 in map_3
    assert map_3.get(2) == "2"
    assert len(map_3) == 3

    map_4 = map_3.remove(2)
    assert 0 in map_4
    assert map_4.get(0) == 0
    assert "1" in map_4
    assert map_4.get("1") == 1
    assert len(map_4) == 2

    assert map_0 != map_1
    assert map_1 != map_2
    assert map_2 != map_3
    assert map_3 != map_4
    assert map_2 == map_4
    assert hash(map_2) == hash(map_4)
