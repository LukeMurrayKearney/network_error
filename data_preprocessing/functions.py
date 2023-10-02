import numpy as np
# import pandas as pd

def egos_for_error_calculation(df, buckets):
    egos = []
    last = ''
    for x in df.iterrows():
        if x[1]['part_id'] == last:
            if np.isnan(x[1]['cnt_rand_age']):
                continue
            else:
                j = -1
                for i, top in enumerate(buckets):
                    if top > x[1]['cnt_rand_age']:
                        j = i
                        break
                egos[-1]['contacts'][j] += 1
        else:
            j = 8
            for i, top in enumerate(buckets):
                if top > x[1]['part_rand_age']:
                    j = i
                    break
            egos.append({'age': j, 'contacts': np.zeros(len(buckets) + 1)})
            if np.isnan(x[1]['cnt_rand_age']):
                continue
            else:
                j = -1
                for i, top in enumerate(buckets):
                    if top > x[1]['cnt_rand_age']:
                        j = i
                        break
                egos[-1]['contacts'][j] += 1
        last = x[1]['part_id']
    print(x[0])
    return egos