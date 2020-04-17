from pyrpds import Vector


def test_vector():
    vector_0 = Vector()
    assert vector_0.first() is None
    assert vector_0.last() is None
    assert len(vector_0) == 0
    assert hash(vector_0) == 0

    vector_1 = vector_0.push_back(0)
    assert vector_1.first() is 0
    assert vector_1.last() is 0
    assert len(vector_1) == 1
    assert hash(vector_1) == 0

    vector_2 = vector_1.push_back("1")
    assert vector_2.first() is 0
    assert vector_2.last() is "1"
    assert len(vector_2) == 2
    assert hash(vector_2) == 0

    vector_3 = vector_2.push_back(2)
    assert vector_3.first() is 0
    assert vector_3.last() is 2
    assert len(vector_3) == 3
    assert hash(vector_3) == 0

    vector_4 = vector_3.drop_last()
    assert vector_4.first() is 0
    assert vector_4.last() is "1"
    assert len(vector_4) == 2
    assert hash(vector_4) == 0

    assert vector_0 != vector_1
    assert vector_1 != vector_2
    assert vector_2 != vector_3
    assert vector_3 != vector_4
    assert vector_2 == vector_4
