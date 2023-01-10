import pandas as pd

bus = pd.read_csv("../MBTA_bus_ridership.csv")
bus = bus.loc[bus.time_period_name == "AM_PEAK"]
bus.loc[bus.stop_id == 64, "stop_name"]

#create a dataframe which focuses solely on routes with a direction of 0
bus0 = bus.loc[bus.direction_id == 0]
stops0 = bus0.groupby("stop_id")["route_name"].unique().apply(list).reset_index(name='routes')
stops_connected0 = stops0.loc[stops0.routes.str.len() >= 2]

#do the same for routes with direction 1
bus1 = bus.loc[bus.direction_id == 1]
stops1 = bus1.groupby("stop_name")["route_name"].unique().apply(list).reset_index(name='routes')
stops_connected1 = stops1.loc[stops1.routes.str.len() >= 2]

routes = bus0.groupby("route_name")["stop_sequence"].unique().apply(list).reset_index(name="sequences")
s_max = bus0.groupby("route_name").agg(s_max = ("stop_sequence", max)).reset_index().values.tolist()

sequence = bus0.groupby("stop_id")["stop_sequence"].unique().apply(list).reset_index(name = "sequence")

list0 = []
for route in bus0.route_name.unique():
    i = bus0.loc[(bus0.route_name == route) & (bus0.stop_sequence == 1), "stop_id"].unique()
    for j in i:
        list0.append(j)
for c in s_max:
    i = bus0.loc[(bus0.route_name == c[0]) & (bus0.stop_sequence == c[1]), "stop_id"].unique()
    for j in i:
        list0.append(j)
for s in stops_connected0.stop_id:
    list0.append(s)

ids0 = pd.DataFrame()
ids0["stop_id"] = list0
sequence_test = bus0.groupby("stop_id")["stop_sequence"].unique().apply(list).reset_index(name='sequence')
s_mean = []
for i in sequence_test.sequence:
    s_mean.append((sum(i)/len(i)).round(3))
sequence_test["s_mean"] = s_mean
sequence0 = sequence_test[["stop_id","s_mean"]]
final0 = ids0.merge(bus0, how = "left", on = "stop_id")
final0 = final0.merge(stops0, how = "left", on = "stop_id")
test = final0.groupby("stop_id").agg({"average_load":"mean"})
test = test.merge(sequence0, how = "left", on = "stop_id")
test = test.merge(stops0, how = "left", on = "stop_id").round(3)
test = test.sort_values(by = "s_mean")
test['connections'] = [list() for x in range(len(test.index))]
test.to_csv("/Users/ericgulottyjr/Downloads/MBTA_Bus_AM_PEAK.csv")