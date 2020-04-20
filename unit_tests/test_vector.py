from pyrpds import pvector


def test_vector():
    vector_0 = pvector()

    try:
        assert vector_0.first() is None
        assert vector_0.last() is None
    except RuntimeError:
        ...

    assert len(vector_0) == 0
    assert hash(vector_0) == -4800647303603446203

    try:
        vector_0.get(-1)
    except OverflowError:
        ...

    try:
        vector_0.get(0)
    except IndexError:
        ...

    try:
        vector_0.set(-1, 0)
    except OverflowError:
        ...

    try:
        vector_0.set(0, 0)
    except IndexError:
        ...

    vector_1 = vector_0.push_back(0)
    assert vector_1.first() is 0
    assert vector_1.get(0) is 0
    assert vector_1.last() is 0
    assert len(vector_1) == 1
    assert hash(vector_1) == -8559946577813192710

    vector_2 = vector_1.push_back("1")
    assert vector_2.first() is 0
    assert vector_2.last() is "1"
    assert vector_2.get(0) is 0
    assert vector_2.get(1) is "1"
    assert len(vector_2) == 2

    vector_3 = vector_2.push_back(2)
    assert vector_3.first() is 0
    assert vector_3.last() is 2
    assert vector_2.get(0) is 0
    assert vector_3.get(1) is "1"
    assert vector_3.get(2) is 2
    assert len(vector_3) == 3

    vector_4 = vector_3.drop_last()
    assert vector_4.first() is 0
    assert vector_4.last() is "1"
    assert vector_4.get(0) is 0
    assert vector_4.get(1) is "1"
    assert len(vector_4) == 2

    vector_5 = vector_4.set(0, "zero")
    assert vector_5.first() is "zero"
    assert vector_5.last() is "1"
    assert vector_5.get(0) is "zero"
    assert vector_5.get(1) is "1"
    assert len(vector_4) == 2

    vector_6 = vector_5.extend([3, "4"])
    assert vector_6.first() is "zero"
    assert vector_6.last() is "4"
    assert vector_6.get(0) is "zero"
    assert vector_6.get(1) is "1"
    assert vector_6.get(2) is 3
    assert vector_6.get(3) is "4"
    assert len(vector_6) == 4

    assert vector_0 != vector_1
    assert vector_1 != vector_2
    assert vector_2 != vector_3
    assert vector_3 != vector_4
    assert vector_2 == vector_4
    assert hash(vector_2) == hash(vector_4)


def test_pvector_constuctor():
    container = pvector()
    assert len(container) == 0

    container = pvector((0, "1", 2))
    assert len(container) == 3

    container = pvector([0, "1", 2])
    assert len(container) == 3

    container = pvector(range(3))
    assert len(container) == 3

    container = pvector(map(lambda x: x, range(3)))
    assert len(container) == 3


def test_iter():
    container = pvector()
    for element in range(100):
        container.push_back(element)
    for index, element in enumerate(sorted(container)):
        assert index == element
