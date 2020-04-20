from pyrpds import plist, l


def test_list():
    list_0 = plist()

    try:
        assert list_0.first() is None
        assert list_0.last() is None
    except RuntimeError:
        ...

    assert len(list_0) == 0
    assert hash(list_0) == -4800647303603446203

    list_1 = list_0.push_front(0)
    assert list_1.first() is 0
    assert list_1.last() is 0
    assert len(list_1) == 1
    assert hash(list_1) == -8559946577813192710

    list_2 = list_1.push_front("1")
    assert list_2.first() is "1"
    assert list_2.last() is 0
    assert len(list_2) == 2

    list_3 = list_2.push_front(2)
    assert list_3.first() is 2
    assert list_3.last() is 0
    assert len(list_3) == 3

    list_4 = list_3.drop_first()
    assert list_4.first() is "1"
    assert list_4.last() is 0
    assert len(list_4) == 2

    assert list_0 != list_1
    assert list_1 != list_2
    assert list_2 != list_3
    assert list_3 != list_4
    assert list_2 == list_4
    assert hash(list_2) == hash(list_4)


def test_plist_constuctor():
    container = plist()
    assert len(container) == 0

    container = plist((0, "1", 2))
    assert len(container) == 3

    container = plist([0, "1", 2])
    assert len(container) == 3

    container = plist(range(5))
    assert len(container) == 5

    container = plist(map(lambda x: x, range(10)))
    assert len(container) == 10


def test_l_constuctor():
    container = l()
    assert len(container) == 0

    container = l(0, "1", 2, 6, 10)
    assert len(container) == 5


def test_iter():
    container = plist()
    for element in range(100):
        container.push_front(element)
    for index, element in enumerate(container):
        assert index == element
