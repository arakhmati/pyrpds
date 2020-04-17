from pyrpds import List


def test_list():
    list_0 = List()
    assert list_0.first() is None
    assert list_0.last() is None
    assert len(list_0) == 0
    assert hash(list_0) == 0

    list_1 = list_0.push_front(0)
    assert list_1.first() is 0
    assert list_1.last() is 0
    assert len(list_1) == 1
    assert hash(list_1) == 0

    list_2 = list_1.push_front("1")
    assert list_2.first() is "1"
    assert list_2.last() is 0
    assert len(list_2) == 2
    assert hash(list_2) == 0

    list_3 = list_2.push_front(2)
    assert list_3.first() is 2
    assert list_3.last() is 0
    assert len(list_3) == 3
    assert hash(list_3) == 0

    list_4 = list_3.drop_first()
    assert list_4.first() is "1"
    assert list_4.last() is 0
    assert len(list_4) == 2
    assert hash(list_4) == 0

    assert list_0 != list_1
    assert list_1 != list_2
    assert list_2 != list_3
    assert list_3 != list_4
    assert list_2 == list_4
