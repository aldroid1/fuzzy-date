import datetime
import dateparser
import fuzzydate
import polars
import timeit

test_cases = [

    # Fixed
    {'source': '1705072948', 'expect': '2024-01-12 15:22:28'},
    {'source': '@1705072948', 'expect': '2024-01-12 15:22:28'},
    {'source': '20230201', 'expect': '2023-02-01 00:00:00'},
    {'source': '2023-02-01', 'expect': '2023-02-01 00:00:00'},
    {'source': 'Feb-01-2023', 'expect': '2023-02-01 00:00:00'},
    {'source': '01-Feb-2023', 'expect': '2023-02-01 00:00:00'},
    {'source': '2023-Feb-01', 'expect': '2023-02-01 00:00:00'},
    {'source': '7.2.2023', 'expect': '2023-02-07 00:00:00'},
    {'source': '2/7/2023', 'expect': '2023-02-07 00:00:00'},
    {'source': 'Dec 7 2023', 'expect': '2023-12-07 00:00:00'},
    {'source': 'Dec. 7th 2023', 'expect': '2023-12-07 00:00:00'},
    {'source': '7th of Dec, 2023', 'expect': '2023-12-07 00:00:00'},
    {'source': '7. Dec 2023', 'expect': '2023-12-07 00:00:00'},
    {'source': '7 December 2023', 'expect': '2023-12-07 00:00:00'},
    {'source': 'Thu Dec 07 02:00:00 2023', 'expect': '2023-12-07 02:00:00'},
    {'source': 'Wed, July 23rd 2008', 'expect': '2008-07-23 00:00:00'},
    {'source': 'Wed, 23rd of July 2008', 'expect': '2008-07-23 00:00:00'},
    {'source': 'Wed, 23 July 2008', 'expect': '2008-07-23 00:00:00'},
    {'source': 'Wed, July 23 2008', 'expect': '2008-07-23 00:00:00'},
    {'source': '2023-12-07 3pm', 'expect': '2023-12-07 15:00:00'},
    {'source': '2023-12-07 15:02:01', 'expect': '2023-12-07 15:02:01'},
    {'source': '2023-12-07T15:02:01', 'expect': '2023-12-07 15:02:01'},
    {'source': '2023-12-07T15:02:01.04', 'expect': '2023-12-07 15:02:01.040000'},
    {'source': '2023-12-07 15:02:01.04', 'expect': '2023-12-07 15:02:01.040000'},
    {'source': 'Week 16', 'expect': '2024-04-15 00:00:00'},
    {'source': 'Week 16, 2024', 'expect': '2024-04-15 00:00:00'},
    {'source': '2024-W16', 'expect': '2024-04-15 00:00:00'},

    # Relative
    {'source': 'now', 'expect': '2024-01-25 15:22:28'},
    {'source': 'today', 'expect': '2024-01-25 00:00:00'},
    {'source': 'yesterday', 'expect': '2024-01-24 00:00:00'},
    {'source': '1 day ago', 'expect': '2024-01-24 15:22:28'},
    {'source': '-1 day', 'expect': '2024-01-24 15:22:28'},
    {'source': '-1 day 2 hours', 'expect': '2024-01-24 13:22:28'},
    {'source': '+1 day 2 hours', 'expect': '2024-01-26 17:22:28'},
    {'source': 'last week', 'expect': '2024-01-15 15:22:28'},
    {'source': 'next week', 'expect': '2024-01-29 15:22:28'},
    {'source': 'last 2 weeks', 'expect': '2024-01-08 15:22:28'},
    {'source': 'last month', 'expect': '2023-12-25 15:22:28'},
    {'source': 'past week', 'expect': '2024-01-18 15:22:28'},
    {'source': 'first day of this month', 'expect': '2024-01-01 00:00:00'},
    {'source': 'first day of February', 'expect': '2024-02-01 00:00:00'},
    {'source': 'first Mon of Feb', 'expect': '2024-02-05 00:00:00'},
    {'source': 'last monday', 'expect': '2024-01-22 00:00:00'},
    {'source': 'prev monday', 'expect': '2024-01-22 00:00:00'},
    {'source': 'tuesday', 'expect': '2024-01-30 00:00:00'},
    {'source': 'december', 'expect': '2024-12-25 00:00:00'},
]

time_now = datetime.datetime(2024, 1, 25, 15, 22, 28, 0)


def check_values():
    for test in test_cases:
        value = dateparser.parse(test['source'], settings={
            'RELATIVE_BASE': time_now,
        })
        test['dateparser'] = str(value) if value else ''

        try:
            value = fuzzydate.to_datetime(
                source=test['source'],
                now=time_now,
                weekday_start_mon=True,
            )
            test['fuzzydate'] = str(value).removesuffix('+00:00')
        except ValueError:
            test['fuzzydate'] = ''

        test['is_match'] = (
                test['dateparser']
                and test['dateparser'] == test['fuzzydate']
        )


def performance_table(iterations: int) -> polars.DataFrame:
    result = [
        {
            'dateparser': min(timeit.repeat(
                lambda: dateparser.parse(test['source']),
                number=iterations,
            )),
            'fuzzydate': min(timeit.repeat(
                lambda: fuzzydate.to_datetime(test['source']),
                number=iterations,
            ))
        }
        for test in test_cases
        if test['is_match']
    ]

    df_table = polars.DataFrame(result, orient='row')

    df_table = df_table.describe().filter(
        polars.col('statistic').is_in(['min', 'max', 'std', 'mean'])
    )

    df_table = df_table.with_columns(
        ((polars.col('dateparser') - polars.col('fuzzydate')) /
         ((polars.col('dateparser') + polars.col('fuzzydate')) / 2) * 100)
        .round(1)
        .alias("diff %")
    )

    return df_table


def discrepancy_table() -> polars.DataFrame:
    result = [
        {
            'test': test['source'],
            'dateparser': test['dateparser'],
            'fuzzydate': test['fuzzydate'],
        }
        for test in test_cases
        if not test['is_match'] and test['dateparser']
    ]

    result.sort(key=lambda i: i['test'])
    return polars.DataFrame(result, orient='row')


def unsupported_table() -> polars.DataFrame:
    result = [
        {
            'test': test['source'],
            'dateparser': '`None`',
            'fuzzydate': test['fuzzydate'],
        }
        for test in test_cases
        if not test['is_match'] and not test['dateparser']
    ]

    result.sort(key=lambda i: i['test'])
    return polars.DataFrame(result, orient='row')


if __name__ == '__main__':
    check_values()

    print('# Tests {}'.format(len(test_cases)))
    print('# Time {}'.format(str(time_now)))

    with polars.Config(
            tbl_rows=100,
            tbl_formatting="MARKDOWN",
            tbl_hide_column_data_types=True,
            tbl_hide_dataframe_shape=True):
        iterations = 100
        p_table = performance_table(iterations)
        d_table = discrepancy_table()
        u_table = unsupported_table()

        print('\n# Performance\n# {} tests, {} iterations\n'.format(
            len(test_cases) - len(d_table) - len(u_table),
            iterations,
        ))
        print(p_table)

        print('\n# Not supported\n# {} tests\n'.format(len(u_table)))
        print(u_table)

        print('\n# Discrepancies\n# {} tests\n'.format(len(d_table)))
        print(d_table)
