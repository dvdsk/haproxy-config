use haproxy_config_parser::parse_sections;

macro_rules! test_file {
    ($name:ident, $path:literal) => {
        #[test]
        fn $name() {
            let file = include_str!(std::concat!("from_gh/", $path, ".cfg"));

            match parse_sections(&file) {
                Err(e) => {
                    let e = e.with_path($path);
                    e.eprint();
                    panic!("{}", e.inner);
                }
                Ok(lines) => println!("{lines:#?}"),
            }
        }
    };
}

test_file!(
    apache_drill_centralized_foreman,
    "apache-drill-centralized-foreman"
);
test_file!(apache_drill, "apache-drill");
test_file!(cassandra_cql, "cassandra-cql");
test_file!(cassandra_jmx, "cassandra-jmx");
test_file!(cassandra_thrift, "cassandra-thrift");
test_file!(chronograf, "chronograf");
test_file!(cloudera_manager, "cloudera-manager");
test_file!(consul, "consul");
test_file!(docker_registry, "docker-registry");
test_file!(docker_swarm, "docker-swarm");
test_file!(elasticsearch_acl, "elasticsearch-acl");
test_file!(elasticsearch_x_pack, "elasticsearch-x-pack");
test_file!(elasticsearch, "elasticsearch");
test_file!(etcd, "etcd");
test_file!(fluentd_forward, "fluentd-forward");
test_file!(fluentd_http, "fluentd-http");
test_file!(fluentd_monitor, "fluentd-monitor");
test_file!(global, "global");
test_file!(grafana, "grafana");
test_file!(graphite_carbon_relay, "graphite-carbon-relay");
test_file!(graphite_web, "graphite-web");
test_file!(h2o, "h2o");
test_file!(hadoop_httpfs, "hadoop-httpfs");
test_file!(hadoop_namenode_2_x, "hadoop-namenode-2.x");
test_file!(hadoop_namenode, "hadoop-namenode");
test_file!(hadoop_yarn_history_server, "hadoop-yarn-history-server");
test_file!(hadoop_yarn_resource_manager, "hadoop-yarn-resource-manager");
test_file!(hbase_master_0_9x, "hbase-master-0.9x");
test_file!(hbase_master, "hbase-master");
test_file!(hbase_stargate_rest_cloudera, "hbase-stargate-rest-cloudera");
test_file!(hbase_stargate_rest, "hbase-stargate-rest");
test_file!(hbase_thrift, "hbase-thrift");
test_file!(hive_metastore, "hive-metastore");
test_file!(hiveserver2_http, "hiveserver2-http");
test_file!(hiveserver2, "hiveserver2");
test_file!(http, "http");
test_file!(hue, "hue");
test_file!(impala_catalog, "impala-catalog");
test_file!(impala_jdbc, "impala-jdbc");
test_file!(impala_odbc, "impala-odbc");
test_file!(impala_statestore, "impala-statestore");
test_file!(impala_ui, "impala-ui");
test_file!(influxdb_relay, "influxdb-relay");
test_file!(influxdb, "influxdb");
test_file!(jenkins, "jenkins");
test_file!(kibana, "kibana");
test_file!(kubernetes_master_apiserver, "kubernetes-master-apiserver");
test_file!(mapr_cldb, "mapr-cldb");
test_file!(mapr_mcs, "mapr-mcs");
test_file!(mariadb, "mariadb");
test_file!(minio, "minio");
test_file!(mysql, "mysql");
test_file!(nifi, "nifi");
test_file!(oozie, "oozie");
test_file!(opentsdb, "opentsdb");
test_file!(postgres, "postgres");
test_file!(presto, "presto");
test_file!(prometheus, "prometheus");
test_file!(rabbitmq_management, "rabbitmq-management");
test_file!(rabbitmq, "rabbitmq");
test_file!(rancher, "rancher");
test_file!(redis_master, "redis-master");
test_file!(redis, "redis");
test_file!(riak, "riak");
test_file!(smtp_ssl, "smtp-ssl");
test_file!(smtp, "smtp");
test_file!(solr, "solr");
test_file!(solrcloud, "solrcloud");
test_file!(spark_history_server, "spark-history-server");
test_file!(sqoop2, "sqoop2");
test_file!(ssh, "ssh");
test_file!(stats, "stats");
test_file!(vault, "vault");
test_file!(zeppelin, "zeppelin");
test_file!(zookeeper_mapr, "zookeeper-mapr");
test_file!(zookeeper, "zookeeper");
